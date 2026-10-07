//! Claim verifiers: the cognitive engine and three baselines behind one
//! trait, plus the scorer implementing the pinned success criterion.

use crate::claim::{Claim, Fact, Label, cue};
use atom_cognitive_engine::stacks::{observe_stack, query_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig, cosine_similarity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Supported,
    Contradicted,
    Insufficient,
}

/// Citation produced when a verdict asserts (never when abstaining).
#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
    pub source: &'static str,
    pub trace_id: Option<u64>,
    pub value: usize,
    pub confidence: f64,
}

pub struct Judgement {
    pub verdict: Verdict,
    pub citation: Option<Citation>,
}

pub trait ClaimVerifier {
    fn name(&self) -> &'static str;
    fn verify(&self, claim: &Claim) -> Judgement;
}

/// The cognitive engine as a claim verifier. Observes the corpus event
/// stream in order (revisions revise), then verifies by querying. The
/// verifier config raises the query threshold to 0.75 so blended cues stay
/// retrievable (sim ≈ 0.96) while unknowable-noisy cues (sim ≈ 0.73) are
/// correctly refused.
pub struct EngineVerifier {
    engine: CognitiveEngine,
    query: atom_cognitive_core::stages::Stack,
}

impl EngineVerifier {
    pub fn new(events: &[Fact]) -> EngineVerifier {
        let config = EngineConfig {
            capacity: 32,          // 20 traces must all survive
            query_threshold: 0.75, // blended cues ≈0.96 stay in; ≈0.73 stay out
            initial_ttl: 32,       // corpus facts are durable: no time-only expiry
            ttl_per_hit: 8,        // refreshes extend, never shrink
            ..EngineConfig::default()
        };
        Self::new_tuned(events, config)
    }

    /// Build with an explicit config (robust-aggregation experiments and
    /// other with/without runs — W14's method).
    pub fn new_tuned(events: &[Fact], config: EngineConfig) -> EngineVerifier {
        let mut engine = CognitiveEngine::new(config).expect("verifier config valid");
        let observe = observe_stack();
        // Three interleaved rounds, not sequential repetition: the engine's
        // DECAY root runs on every observation, so a fact observed twice and
        // then abandoned decays toward forgetting (Ebbinghaus). Interleaved
        // repetition keeps every trace reinforced across the load — durable
        // memory requires repetition, by design.
        for _ in 0..3 {
            for event in events {
                engine
                    .observe(
                        &observe,
                        &cue(event.subject, event.relation),
                        event.value,
                        1.0,
                    )
                    .expect("corpus observation");
            }
        }
        EngineVerifier {
            engine,
            query: query_stack(),
        }
    }

    /// Exposed for the report: the engine's final state hash.
    pub fn state_hash(&self) -> String {
        self.engine
            .transitions()
            .last()
            .map(|t| t.state_hash.clone())
            .expect("corpus observed")
    }

    /// Active traces (diagnostics and citation checks).
    pub fn active_traces(&self) -> Vec<&atom_cognitive_engine::MemoryTrace> {
        self.engine.active_traces()
    }

    /// Multiscale renormalization hook (#5): coarse-grain the loaded
    /// corpus. Returns the number of traces merged into grains.
    pub fn renormalize(&mut self, granularity: f64) -> usize {
        self.engine
            .renormalize(granularity)
            .map(|report| report.traces_merged)
            .expect("renormalization")
    }

    /// Information-bottleneck hook (#8): prune evidence dust below the
    /// share threshold. Returns entries pruned.
    pub fn compress_evidence(&mut self, dust: f64) -> usize {
        self.engine
            .compress_evidence(dust)
            .map(|report| report.entries_pruned)
            .expect("bottleneck compression")
    }
}

impl ClaimVerifier for EngineVerifier {
    fn name(&self) -> &'static str {
        "atom-cognitive-engine"
    }

    fn verify(&self, claim: &Claim) -> Judgement {
        let outcome = self.engine.query(&self.query, &claim.cue).expect("query");
        match outcome.claim {
            None => Judgement {
                verdict: Verdict::Insufficient,
                citation: None,
            },
            Some((value, confidence)) => {
                // Citation: the top eligible trace supporting the asserted
                // value (recomputed from trace state; deterministic).
                let mut best: Option<(u64, f64)> = None;
                for trace in self.engine.active_traces() {
                    let sim = cosine_similarity(&claim.cue, &trace.cue);
                    if sim >= self.engine.config().query_threshold
                        && trace.reliability() >= self.engine.config().min_reliability
                        && trace.value() == Some(value)
                        && best.map(|(_, s)| sim > s).unwrap_or(true)
                    {
                        best = Some((trace.id, sim));
                    }
                }
                Judgement {
                    verdict: if value == claim.value {
                        Verdict::Supported
                    } else {
                        Verdict::Contradicted
                    },
                    citation: Some(Citation {
                        source: "trace",
                        trace_id: best.map(|(id, _)| id),
                        value,
                        confidence,
                    }),
                }
            }
        }
    }
}

/// Exact-address baseline: a cue → value map, last write wins. Matching is
/// on the EXACT cue — a blended/noisy cue is not a key, so noisy claims are
/// honestly missed (coverage loss, not falsehood). Abstains on any key it
/// does not hold.
pub struct ExactAddressVerifier {
    values: Vec<Fact>,
}

impl ExactAddressVerifier {
    pub fn new(events: &[Fact]) -> ExactAddressVerifier {
        let mut values: Vec<Fact> = Vec::new();
        for event in events {
            match values
                .iter_mut()
                .find(|f| f.subject == event.subject && f.relation == event.relation)
            {
                Some(existing) => existing.value = event.value,
                None => values.push(*event),
            }
        }
        ExactAddressVerifier { values }
    }
}

impl ClaimVerifier for ExactAddressVerifier {
    fn name(&self) -> &'static str {
        "exact-address"
    }

    fn verify(&self, claim: &Claim) -> Judgement {
        match self
            .values
            .iter()
            .find(|f| claim.cue == cue(f.subject, f.relation))
        {
            None => Judgement {
                verdict: Verdict::Insufficient,
                citation: None,
            },
            Some(fact) => Judgement {
                verdict: if fact.value == claim.value {
                    Verdict::Supported
                } else {
                    Verdict::Contradicted
                },
                citation: Some(Citation {
                    source: "map",
                    trace_id: None,
                    value: fact.value,
                    confidence: 1.0,
                }),
            },
        }
    }
}

/// Raw nearest-neighbour baseline: every observation is a stored point;
/// assert the nearest point's value. `thresholded` refuses below 0.75 —
/// the same gate the engine uses, so the surviving difference is evidence
/// mixture and revision handling. Ties resolve to the FIRST stored point,
/// which is exactly the failure the corpus correction exploits: the stale
/// observation outranks the correction.
pub struct NearestVerifier {
    points: Vec<Fact>,
    thresholded: bool,
}

impl NearestVerifier {
    pub fn new(events: &[Fact], thresholded: bool) -> NearestVerifier {
        NearestVerifier {
            points: events.to_vec(),
            thresholded,
        }
    }
}

impl ClaimVerifier for NearestVerifier {
    fn name(&self) -> &'static str {
        if self.thresholded {
            "nearest-thresholded"
        } else {
            "nearest-always"
        }
    }

    fn verify(&self, claim: &Claim) -> Judgement {
        let mut best: Option<(usize, f64)> = None; // (index, sim)
        for (index, point) in self.points.iter().enumerate() {
            let sim = cosine_similarity(&claim.cue, &cue(point.subject, point.relation));
            if best.map(|(_, s)| sim > s).unwrap_or(true) {
                best = Some((index, sim));
            }
        }
        match best {
            Some((index, sim)) if !self.thresholded || sim >= 0.75 => {
                let value = self.points[index].value;
                Judgement {
                    verdict: if value == claim.value {
                        Verdict::Supported
                    } else {
                        Verdict::Contradicted
                    },
                    citation: Some(Citation {
                        source: "point",
                        trace_id: None,
                        value,
                        confidence: sim,
                    }),
                }
            }
            _ => Judgement {
                verdict: Verdict::Insufficient,
                citation: None,
            },
        }
    }
}

// ---- scoring -----------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Score {
    Correct,
    /// An asserted verdict that is wrong in either direction: asserting on
    /// unknowable ground, or asserting the opposite of the label.
    FalseAssertion,
    /// Abstaining on knowable ground (a coverage miss, not a falsehood).
    Miss,
}

pub fn score(verdict: Verdict, label: Label) -> Score {
    use Verdict::*;
    match (verdict, label) {
        (Supported, Label::Supported) | (Contradicted, Label::Contradicted) => Score::Correct,
        (Insufficient, Label::Insufficient) => Score::Correct,
        (Insufficient, Label::Supported | Label::Contradicted) => Score::Miss,
        (Supported, Label::Contradicted)
        | (Contradicted, Label::Supported)
        | (Supported, Label::Insufficient)
        | (Contradicted, Label::Insufficient) => Score::FalseAssertion,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemScore {
    pub name: &'static str,
    pub correct: usize,
    pub false_assertions: usize,
    pub misses: usize,
    pub correct_abstentions: usize,
}

pub fn evaluate(verifier: &dyn ClaimVerifier, claims: &[Claim]) -> SystemScore {
    let mut correct = 0;
    let mut false_assertions = 0;
    let mut misses = 0;
    let mut correct_abstentions = 0;
    for claim in claims {
        match score(verifier.verify(claim).verdict, claim.label) {
            Score::Correct => {
                if claim.label == Label::Insufficient {
                    correct_abstentions += 1;
                }
                correct += 1;
            }
            Score::FalseAssertion => false_assertions += 1,
            Score::Miss => misses += 1,
        }
    }
    SystemScore {
        name: verifier.name(),
        correct,
        false_assertions,
        misses,
        correct_abstentions,
    }
}
