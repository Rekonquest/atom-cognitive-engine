//! Cognitive RAG: AtomDB recalls, the cognitive engine judges.
//!
//! ```text
//! question ──► AtomDB retrieval field ──► shortlist (cue intersections, threads)
//!                                              │
//!              feedback facts (AtomDB) ──► deliberation engine (one per ask)
//!                                              │  window traces, primed by
//!                                              │  replayed helpful/misleading facts
//!                                              ▼
//!                               gate → retrieve → assert-or-abstain
//!                                              │
//!                     reciprocal-rank fusion ◄─┘
//!                                              ▼
//!                       Answer { verdict, cited evidence, both lanes' scores }
//! ```
//!
//! Division of authority:
//! - **AtomDB owns truth.** Passages, sources and feedback are immutable,
//!   content-addressed facts. Recall is read-only; `feedback` is the only
//!   write path and it is an explicit durable fact (AtomDB Stage 9).
//! - **The engine owns judgment.** It never writes to AtomDB. Each ask builds
//!   a fresh, bounded engine from the shortlist and the feedback facts, so its
//!   verdict is a deterministic function of the durable store and the query.
//!   Misleading feedback becomes a contradiction the engine revises on;
//!   the engine's gate and abstention decide whether the shortlist actually
//!   answers the question or only shares words with it.

pub mod embed;
pub mod json;

use atom_cognitive_engine::stacks::{observe_stack, query_stack, record_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig, FluencyState};
use atom_db::{AtomDb, ContextPacket, Digest, Polarity, RetrievalConfig, Retriever};
use std::collections::BTreeMap;
use std::path::Path;

/// Engine value lane: what accumulated evidence says about a passage.
pub const HELPFUL: usize = 0;
pub const MISLEADING: usize = 1;

const FEEDBACK_PREFIX: &str = "atom-db/retrieval/feedback/v1/";
/// Reciprocal-rank-fusion constant (the standard k = 60).
const RRF_K: f64 = 60.0;

#[derive(Debug)]
pub enum RagError {
    Store(atom_db::Error),
    Engine(String),
    Config(String),
}

impl std::fmt::Display for RagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RagError::Store(e) => write!(f, "atomdb: {e}"),
            RagError::Engine(e) => write!(f, "engine: {e}"),
            RagError::Config(e) => write!(f, "config: {e}"),
        }
    }
}

impl std::error::Error for RagError {}

impl From<atom_db::Error> for RagError {
    fn from(e: atom_db::Error) -> Self {
        RagError::Store(e)
    }
}

#[derive(Debug, Clone)]
pub struct RagConfig {
    pub retrieval: RetrievalConfig,
    pub engine: EngineConfig,
    /// How many fused passages an answer cites.
    pub max_citations: usize,
}

impl Default for RagConfig {
    fn default() -> Self {
        let retrieval = RetrievalConfig {
            // A wider shortlist than plain AtomDB: the engine is the filter.
            max_evidence: 16,
            max_context_bytes: 32 * 1024,
            ..RetrievalConfig::default()
        };
        // Deliberation regime, not a memory experiment: the engine lives for
        // one ask, so conservation and decay must never eat the shortlist.
        let engine = EngineConfig {
            cue_dim: embed::DIM,
            value_count: 2,
            capacity: 1024,
            initial_ttl: 512,
            ttl_per_hit: 16,
            decay_support_rate: 0.0005,
            decay_mass_rate: 0.0003,
            conservation_budget: 1.0e6,
            // Hashed vectors are coarse; the gate stays at its floor.
            query_threshold: 0.5,
            // Overlapping AtomDB windows produce near-copy windows;
            // correlated evidence is not independent evidence (W17).
            dedup_threshold: 0.97,
            ..EngineConfig::default()
        };
        RagConfig {
            retrieval,
            engine,
            max_citations: 5,
        }
    }
}

/// Why the system declined to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstainReason {
    /// AtomDB found no passage the question's cues converge on.
    NoEvidence,
    /// AtomDB found word overlap, but no shortlisted window passed the
    /// engine's gate: the shortlist does not answer the question.
    EngineUncertain,
    /// The eligible evidence is dominated by passages marked misleading.
    Misleading,
}

impl AbstainReason {
    pub fn describe(self) -> &'static str {
        match self {
            AbstainReason::NoEvidence => "no evidence in the store",
            AbstainReason::EngineUncertain => "evidence shares words but does not answer",
            AbstainReason::Misleading => "evidence is dominated by passages marked misleading",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Grounded { confidence: f64 },
    Abstain(AbstainReason),
}

#[derive(Debug, Clone)]
pub struct Citation {
    pub passage: Digest,
    pub sources: Vec<String>,
    pub text: String,
    pub fused_score: f64,
    /// 1-based rank in AtomDB's field.
    pub atomdb_rank: usize,
    pub atomdb_activation: u64,
    pub supporting_cues: Vec<String>,
    /// 1-based rank among engine-activated passages (None = gated out).
    pub engine_rank: Option<usize>,
    /// Engine mixture weight of the passage's best window (0 if not in
    /// the top activations).
    pub engine_weight: f64,
    pub feedback: (u64, u64),
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub query: String,
    pub verdict: Verdict,
    pub citations: Vec<Citation>,
    pub shortlist: usize,
    pub window_traces: usize,
    pub eligible_traces: usize,
    pub fluency: Option<FluencyState>,
    pub snapshot_sequence: u64,
    pub response_hash: String,
}

pub struct CognitiveRag {
    db: AtomDb,
    retriever: Retriever,
    config: RagConfig,
}

impl CognitiveRag {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RagError> {
        Self::open_with(path, RagConfig::default(), false)
    }

    pub fn open_read_only(path: impl AsRef<Path>) -> Result<Self, RagError> {
        Self::open_with(path, RagConfig::default(), true)
    }

    pub fn open_with(
        path: impl AsRef<Path>,
        config: RagConfig,
        read_only: bool,
    ) -> Result<Self, RagError> {
        config.engine.validate().map_err(RagError::Config)?;
        if config.engine.cue_dim != embed::DIM || config.engine.value_count < 2 {
            return Err(RagError::Config(
                "engine needs cue_dim = embed::DIM and two value lanes".into(),
            ));
        }
        let retriever = Retriever::new(config.retrieval).map_err(RagError::Config)?;
        let db = if read_only {
            AtomDb::open_read_only(path)?
        } else {
            AtomDb::open_writer(path)?
        };
        Ok(CognitiveRag {
            db,
            retriever,
            config,
        })
    }

    pub fn db(&mut self) -> &mut AtomDb {
        &mut self.db
    }

    /// Durable ingestion: one atomic AtomDB cell per document.
    pub fn remember(&mut self, source: &str, text: &str) -> Result<Vec<Digest>, RagError> {
        Ok(self
            .retriever
            .remember(&mut self.db, source, text)?
            .passages)
    }

    /// Record whether a cited passage helped. Durable (an AtomDB fact), so it
    /// reshapes both lanes on every later ask: AtomDB shifts the passage's
    /// conductance, and the engine replays it as confirmation/contradiction.
    pub fn feedback(&mut self, passage: Digest, helpful: bool) -> Result<u64, RagError> {
        let polarity = if helpful {
            Polarity::Strengthen
        } else {
            Polarity::Weaken
        };
        Ok(self
            .retriever
            .reinforce(&mut self.db, passage, polarity)?
            .count)
    }

    /// Latest (helpful, misleading) counts for a passage, read from the
    /// durable feedback facts (the append-only history is the counter).
    pub fn feedback_counts(&mut self, passage: Digest) -> Result<(u64, u64), RagError> {
        let mut counts = (0u64, 0u64);
        for (_, bond) in self.db.bonds_from(passage) {
            let Some(bytes) = self.db.get_atom(bond.relation)? else {
                continue;
            };
            let Some(rest) = std::str::from_utf8(&bytes)
                .ok()
                .and_then(|r| r.strip_prefix(FEEDBACK_PREFIX))
            else {
                continue;
            };
            let mut parts = rest.split('/');
            let (Some(slug), Some(Ok(count))) = (parts.next(), parts.next().map(str::parse::<u64>))
            else {
                continue;
            };
            match slug {
                "strengthen" => counts.0 = counts.0.max(count),
                "weaken" => counts.1 = counts.1.max(count),
                _ => {}
            }
        }
        Ok(counts)
    }

    /// Recall lane alone (AtomDB's context packet), for inspection.
    pub fn recall(&mut self, query: &str) -> Result<ContextPacket<Digest>, RagError> {
        Ok(self.retriever.retrieve(&mut self.db, query)?)
    }

    pub fn ask(&mut self, query: &str) -> Result<Answer, RagError> {
        let packet = self.recall(query)?;
        let mut answer = Answer {
            query: query.to_string(),
            verdict: Verdict::Abstain(AbstainReason::NoEvidence),
            citations: Vec::new(),
            shortlist: packet.evidence.len(),
            window_traces: 0,
            eligible_traces: 0,
            fluency: None,
            snapshot_sequence: packet.snapshot_sequence,
            response_hash: String::new(),
        };
        let query_cue = embed::embed(query);
        if packet.evidence.is_empty() || query_cue.is_none() {
            return Ok(answer);
        }

        // 1. Deliberation workspace: one engine per ask, built only from
        //    durable facts, so the verdict is replayable.
        let engine_err = |e: atom_cognitive_engine::EngineError| RagError::Engine(format!("{e:?}"));
        let mut engine = CognitiveEngine::new(self.config.engine.clone()).map_err(engine_err)?;
        let record = record_stack();
        let observe = observe_stack();
        let mut owner: BTreeMap<u64, usize> = BTreeMap::new();
        let mut feedback = Vec::with_capacity(packet.evidence.len());
        for (index, item) in packet.evidence.iter().enumerate() {
            // Judge the whole passage, not the context-budget excerpt.
            let full = match self.db.get_atom(item.identity)? {
                Some(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                None => item.text.clone(),
            };
            let counts = self.feedback_counts(item.identity)?;
            feedback.push(counts);
            // The source label is evidence of topic too (titles are what
            // AtomDB passages lack in their own text): one extra window.
            let mut windows = embed::windows(&full);
            for source in &item.sources {
                let words = embed::content_words(source);
                if !words.is_empty() {
                    windows.push(words);
                }
            }
            for window in windows {
                if owner.len() >= self.config.engine.capacity {
                    break;
                }
                let Some(cue) = embed::embed_words(&window) else {
                    continue;
                };
                engine
                    .observe(&record, &cue, HELPFUL, 1.0)
                    .map_err(engine_err)?;
                let id = engine.traces().last().map(|t| t.id).unwrap_or_default();
                owner.insert(id, index);
                // 2. Replay durable feedback: confirmations reinforce,
                //    misleading marks are contradictions the engine revises on.
                for _ in 0..counts.0 {
                    engine
                        .observe(&observe, &cue, HELPFUL, 1.0)
                        .map_err(engine_err)?;
                }
                for _ in 0..counts.1 {
                    engine
                        .observe(&observe, &cue, MISLEADING, 1.0)
                        .map_err(engine_err)?;
                }
            }
        }
        answer.window_traces = owner.len();

        // 3. Judgment: gate → retrieve → assert-or-abstain.
        let outcome = engine
            .query(&query_stack(), query_cue.as_deref().unwrap_or_default())
            .map_err(engine_err)?;
        answer.eligible_traces = outcome.eligible_count;
        answer.fluency = Some(outcome.fluency.state);
        answer.response_hash = outcome.response_hash.clone();

        let mut engine_rank: BTreeMap<usize, usize> = BTreeMap::new();
        for id in &outcome.activated_all {
            if let Some(&passage) = owner.get(id) {
                let next = engine_rank.len() + 1;
                engine_rank.entry(passage).or_insert(next);
            }
        }
        let mut engine_weight: BTreeMap<usize, f64> = BTreeMap::new();
        for (id, weight) in &outcome.activations {
            if let Some(&passage) = owner.get(id) {
                let w = engine_weight.entry(passage).or_insert(0.0);
                *w = w.max(*weight);
            }
        }

        // 4. Fusion: reciprocal ranks from both lanes. A passage the engine
        //    gated out keeps only its AtomDB term.
        let mut citations: Vec<Citation> = packet
            .evidence
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let erank = engine_rank.get(&index).copied();
                let fused = 1.0 / (RRF_K + (index + 1) as f64)
                    + erank.map_or(0.0, |r| 1.0 / (RRF_K + r as f64));
                Citation {
                    passage: item.identity,
                    sources: item.sources.clone(),
                    text: item.text.clone(),
                    fused_score: fused,
                    atomdb_rank: index + 1,
                    atomdb_activation: item.activation,
                    supporting_cues: item.supporting_cues.clone(),
                    engine_rank: erank,
                    engine_weight: engine_weight.get(&index).copied().unwrap_or(0.0),
                    feedback: feedback[index],
                }
            })
            .collect();
        citations.sort_by(|a, b| {
            b.fused_score
                .total_cmp(&a.fused_score)
                .then(a.atomdb_rank.cmp(&b.atomdb_rank))
        });

        answer.verdict = match outcome.claim {
            Some((HELPFUL, confidence)) => Verdict::Grounded { confidence },
            Some(_) => Verdict::Abstain(AbstainReason::Misleading),
            None => Verdict::Abstain(AbstainReason::EngineUncertain),
        };
        if matches!(answer.verdict, Verdict::Grounded { .. }) {
            // A grounded answer cites only what passed the gate, minus
            // passages the evidence has turned against.
            citations.retain(|c| c.engine_rank.is_some() && c.feedback.1 <= c.feedback.0);
        }
        citations.truncate(self.config.max_citations);
        answer.citations = citations;
        Ok(answer)
    }
}
