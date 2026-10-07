//! Mother corpus loader: ingests the real 195-memory constitution into the
//! cognitive engine in record mode, wires echo bonds, and answers scenario
//! queries with cited activations and behavior mixtures.
//!
//! Cue encoding (20 dims): bucket one-hot (11) + dominance one-hot (3) +
//! cast presence (6: grandmother, director, protectee, her_mother,
//! stepfather, other). Behavior codes map to evidence indices by sorted
//! order — deterministic and recorded in the manifest.
//!
//! `MotherSystem` is the two-lane assembly: the identity lane (episodic
//! memories, facet+bond retrieval) and the reference lane (the sourced,
//! read-only knowledge pack). The lanes answer side by side and NEVER
//! merge into one confidence score — the atom-harness two-lane law.

use crate::minijson::{Json, parse};
use atom_cognitive_engine::stacks::{mother_query_stack, record_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig, QueryTuning, TraceTier};
use std::collections::HashMap;
use std::path::Path;

pub const CUE_DIM: usize = 20;
pub const DOMINANCES: [&str; 3] = ["identity_dominant", "operational_dominant", "integrated"];
pub const CASTS: [&str; 6] = [
    "grandmother",
    "director",
    "protectee",
    "her_mother",
    "stepfather",
    "other",
];

/// A scenario's situational facets (what is present, which mode is active).
pub struct Scenario {
    pub bucket: usize,
    pub dominance: usize,
    pub casts: Vec<usize>,
}

pub struct ScenarioOutcome {
    pub abstained: bool,
    /// Top activations: (memory id, memory title, mixture weight).
    pub activations: Vec<(String, String, f64)>,
    /// Every activated memory id, ranked by weight (reachability surface).
    pub all_activated: Vec<String>,
    /// Behavior mixture ranked by aggregated weight: (code, weight).
    pub behaviors: Vec<(String, f64)>,
    pub response_hash: String,
}

pub struct MotherCorpus {
    pub engine: CognitiveEngine,
    /// memory id (e.g. "ANCHOR-001") → trace id
    trace_of: HashMap<String, u64>,
    /// trace id → (memory id, title)
    memory_of: HashMap<u64, (String, String)>,
    /// behavior code → evidence index
    behavior_index: Vec<String>,
    pub memories: usize,
    pub drafts_skipped: usize,
    pub bonds_added: usize,
    pub bonds_skipped_dangling: usize,
    pub load_bearing: usize,
}

fn normalize_cast(raw: &str) -> usize {
    let lowered: String = raw.trim_matches(|c| c == '[' || c == ']').to_lowercase();
    match lowered.as_str() {
        "grandmother" => 0,
        "director" => 1,
        "protectee" => 2,
        "her_mother" => 3,
        "stepfather" => 4,
        _ => 5,
    }
}

fn scenario_cue(scenario: &Scenario) -> Vec<f64> {
    let mut cue = vec![0.0; CUE_DIM];
    if (1..=11).contains(&scenario.bucket) {
        cue[scenario.bucket - 1] = 1.0;
    }
    cue[11 + scenario.dominance % 3] = 1.0;
    for cast in &scenario.casts {
        cue[14 + cast % 6] = 1.0;
    }
    cue
}

fn memory_cue(bucket: i64, dominance: &str, casts: &[String]) -> Result<Vec<f64>, String> {
    if !(1..=11).contains(&bucket) {
        return Err(format!("bucket {bucket} out of range"));
    }
    let dominance_index = DOMINANCES
        .iter()
        .position(|d| *d == dominance)
        .ok_or_else(|| format!("unknown dominance {dominance:?}"))?;
    let mut cue = vec![0.0; CUE_DIM];
    cue[(bucket - 1) as usize] = 1.0;
    cue[11 + dominance_index] = 1.0;
    for cast in casts {
        cue[14 + normalize_cast(cast)] = 1.0;
    }
    if cue.iter().all(|v| *v == 0.0) {
        return Err("memory cue is empty".into());
    }
    Ok(cue)
}

fn string_list(value: Option<&Json>, field: &str) -> Result<Vec<String>, String> {
    match value {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(String::from)
                    .ok_or_else(|| format!("{field} entries must be strings"))
            })
            .collect(),
        _ => Err(format!("field {field:?} must be an array of strings")),
    }
}

impl MotherCorpus {
    /// Load and ingest a corpus file (the 195-memory JSON array).
    pub fn load(path: &Path) -> Result<MotherCorpus, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read corpus {path:?}: {error}"))?;
        let root = parse(&text).map_err(|error| format!("corpus JSON invalid: {error}"))?;
        let items = root
            .as_array()
            .ok_or_else(|| "corpus must be a JSON array".to_string())?;
        if items.len() > 2000 {
            return Err(format!("corpus size {} implausible", items.len()));
        }

        // Blueprint discipline (D31): only APPROVED memories load. Drafts
        // are installed on disk but never ingested — an unreviewed memory
        // is not part of who she is.
        let approved: Vec<&Json> = items
            .iter()
            .filter(|item| item.get("review_status").and_then(Json::as_str) == Some("approved"))
            .collect();
        if approved.is_empty() {
            return Err("corpus contains no approved memories".into());
        }
        let drafts_skipped = items.len() - approved.len();

        // Deterministic behavior index: sorted unique codes (approved only).
        let mut codes: Vec<String> = Vec::new();
        for item in &approved {
            for code in string_list(item.get("behaviors_encoded"), "behaviors_encoded")? {
                if !codes.contains(&code) {
                    codes.push(code);
                }
            }
        }
        codes.sort();
        if codes.len() < 2 {
            return Err("corpus encodes fewer than two behaviors".into());
        }

        let config = EngineConfig {
            cue_dim: CUE_DIM,
            value_count: codes.len(),
            capacity: 256,
            // Constitution regime: durable records, near-zero decay, no
            // conservation pressure during load.
            initial_ttl: 512,
            ttl_per_hit: 16,
            decay_support_rate: 0.0005,
            decay_mass_rate: 0.0003,
            conservation_budget: 1.0e6,
            query_threshold: 0.60,
            ..EngineConfig::default()
        };
        let mut engine = CognitiveEngine::new(config).map_err(|e| e.to_string())?;
        let record = record_stack();
        let mut trace_of: HashMap<String, u64> = HashMap::new();
        let mut memory_of: HashMap<u64, (String, String)> = HashMap::new();
        let mut load_bearing = 0usize;

        for item in &approved {
            let id = item
                .get("id")
                .and_then(Json::as_str)
                .ok_or("memory missing string id")?
                .to_string();
            let title = item
                .get("title")
                .and_then(Json::as_str)
                .unwrap_or("")
                .to_string();
            let bucket = item
                .get("bucket")
                .and_then(Json::as_int)
                .ok_or_else(|| format!("memory {id} missing int bucket"))?;
            let dominance = item
                .get("dominance")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("memory {id} missing dominance"))?
                .to_string();
            let is_load_bearing = item
                .get("load_bearing")
                .and_then(Json::as_bool)
                .ok_or_else(|| format!("memory {id} missing load_bearing bool"))?;
            let behaviors = string_list(item.get("behaviors_encoded"), "behaviors_encoded")?;
            let casts = string_list(item.get("cast_present"), "cast_present")?;
            // Validate the field shape now; bonds are wired in pass two.
            string_list(item.get("echoes_to"), "echoes_to")?;
            let cue = memory_cue(bucket, &dominance, &casts)
                .map_err(|error| format!("memory {id}: {error}"))?;
            let mut indices: Vec<usize> = behaviors
                .iter()
                .map(|code| {
                    codes
                        .iter()
                        .position(|c| c == code)
                        .ok_or_else(|| format!("behavior {code} not indexed"))
                })
                .collect::<Result<_, _>>()?;
            indices.sort_unstable();
            indices.dedup();
            if indices.is_empty() {
                return Err(format!("memory {id} encodes no behaviors"));
            }
            engine.set_ingest_tier(if is_load_bearing {
                load_bearing += 1;
                TraceTier::Core
            } else {
                TraceTier::Standard
            });
            let salience = if is_load_bearing { 1.4 } else { 0.75 };
            engine
                .observe_values(&record, &cue, &indices, salience)
                .map_err(|error| format!("memory {id}: {error}"))?;
            // The record stack always nucleates: the newest trace is this
            // memory's trace.
            let trace_id = engine.traces().last().expect("nucleated").id;
            trace_of.insert(id.clone(), trace_id);
            memory_of.insert(trace_id, (id.clone(), title.clone()));
        }

        // Second pass: wire echo bonds (skip dangling targets — they point
        // into drafts or the full 1,950-memory corpus we do not load).
        let mut bonds_added = 0usize;
        let mut bonds_skipped = 0usize;
        for item in &approved {
            let source = item
                .get("id")
                .and_then(Json::as_str)
                .expect("validated above");
            for target in string_list(item.get("echoes_to"), "echoes_to")? {
                let from = trace_of[source];
                match trace_of.get(&target) {
                    Some(&to) => {
                        engine
                            .add_bond(from, to)
                            .map_err(|error| format!("bond {source}->{target}: {error}"))?;
                        bonds_added += 1;
                    }
                    None => bonds_skipped += 1,
                }
            }
        }

        Ok(MotherCorpus {
            engine,
            trace_of,
            memory_of,
            behavior_index: codes,
            memories: approved.len(),
            drafts_skipped,
            bonds_added,
            bonds_skipped_dangling: bonds_skipped,
            load_bearing,
        })
    }

    /// Placebo loader (D31): identical memories, identical bond COUNT, but
    /// the echo edges are wired to RANDOM pairs among the loaded traces.
    /// If real echo links carry activation better than random wiring, the
    /// bridge is real; if not, the graph shape is decorative.
    pub fn load_with_shuffled_bonds(path: &Path, seed: u64) -> Result<MotherCorpus, String> {
        let mut corpus = MotherCorpus::load(path)?;
        let real_bonds = corpus.bonds_added;
        // Unwire the real bonds: rebuild the engine-side bond list by
        // removing all bonds, then re-adding the same count at random.
        corpus.engine.clear_bonds();
        let trace_ids: Vec<u64> = corpus.trace_of.values().copied().collect();
        if trace_ids.len() < 2 {
            return Err("cannot shuffle bonds with fewer than two traces".into());
        }
        let mut state = seed.wrapping_mul(0x9E3779B97F4A7C15) | 1;
        let next = |state: &mut u64| -> u64 {
            let mut x = *state;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            *state = x;
            x
        };
        let mut wired = 0usize;
        let mut attempts = 0usize;
        while wired < real_bonds && attempts < real_bonds * 40 + 1000 {
            attempts += 1;
            let left = trace_ids[(next(&mut state) as usize) % trace_ids.len()];
            let right = trace_ids[(next(&mut state) as usize) % trace_ids.len()];
            if left == right {
                continue;
            }
            if corpus.engine.add_bond(left, right).is_ok() {
                wired += 1;
            }
        }
        corpus.bonds_added = wired;
        Ok(corpus)
    }

    pub fn trace_id_of(&self, memory_id: &str) -> Option<u64> {
        self.trace_of.get(memory_id).copied()
    }

    pub fn behavior_code(&self, index: usize) -> Option<&str> {
        self.behavior_index.get(index).map(String::as_str)
    }

    pub fn behavior_codes(&self) -> &[String] {
        &self.behavior_index
    }

    /// Query a scenario with the engine's default knobs.
    pub fn query(&self, scenario: &Scenario) -> Result<ScenarioOutcome, String> {
        self.query_tuned(scenario, QueryTuning::from_config(self.engine.config()))
    }

    /// Query a scenario with per-query knob overrides (clamped to safety
    /// floors before use — the engine's config and state are untouched).
    /// The with/without sweep surface and the governor's control path for
    /// Mother: she may change HOW the corpus is read, never WHAT it says.
    pub fn query_tuned(
        &self,
        scenario: &Scenario,
        tuning: QueryTuning,
    ) -> Result<ScenarioOutcome, String> {
        let cue = scenario_cue(scenario);
        let outcome = self
            .engine
            .query_tuned(&mother_query_stack(), &cue, tuning)
            .map_err(|e| e.to_string())?;
        let abstained = outcome.claim.is_none() && outcome.activations.is_empty();
        let mut activations: Vec<(String, String, f64)> = Vec::new();
        let mut behavior_weights: Vec<f64> = vec![0.0; self.behavior_index.len()];
        for (trace_id, weight) in &outcome.activations {
            let (memory_id, title) = self
                .memory_of
                .get(trace_id)
                .cloned()
                .unwrap_or_else(|| (trace_id.to_string(), String::new()));
            activations.push((memory_id, title, *weight));
        }
        // Behavior mixture aggregates over the FULL activation set (not
        // just the citation top-8) — crowded facet cues activate dozens of
        // memories and the mixture is the honest summary. Top-8 members
        // contribute their mixture weight; deeper members a fixed small
        // presence credit.
        for trace_id in &outcome.activated_all {
            let Some(trace) = self.engine.trace_by_id(*trace_id) else {
                return Err(format!("unknown trace {trace_id}"));
            };
            let credit = outcome
                .activations
                .iter()
                .find(|(id, _)| id == trace_id)
                .map(|(_, w)| *w)
                .unwrap_or(0.02);
            let distribution = trace.value_distribution();
            for (index, density) in distribution.iter().enumerate() {
                behavior_weights[index] += credit * density;
            }
        }
        let mut behaviors: Vec<(String, f64)> = self
            .behavior_index
            .iter()
            .cloned()
            .zip(behavior_weights)
            .filter(|(_, weight)| *weight > 1e-6)
            .collect();
        behaviors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        behaviors.truncate(12);
        Ok(ScenarioOutcome {
            abstained,
            all_activated: outcome
                .activated_all
                .iter()
                .filter_map(|id| self.memory_of.get(id).map(|(mid, _)| mid.clone()))
                .collect(),
            activations,
            behaviors,
            response_hash: outcome.response_hash,
        })
    }
}

/// The two-lane system: identity (episodic memories) + reference (sourced
/// facts). Answers carry both lanes, separately — never one merged score.
pub struct MotherSystem {
    pub corpus: MotherCorpus,
    pub reference: crate::reference::ReferencePack,
}

/// A two-lane answer to a topic question.
pub struct AskOutcome {
    /// Reference-lane claims with their sources: (claim, source title, url).
    pub claims: Vec<(String, String, String)>,
    /// Whether any reference claim on this topic is flagged urgent.
    pub urgent: bool,
    /// Identity-lane behaviors a protectee-present scenario activates.
    pub behaviors: Vec<(String, f64)>,
    /// Identity-lane top memories (citations).
    pub memories: Vec<(String, String)>,
    pub disclaimer: String,
}

impl MotherSystem {
    pub fn load(corpus_path: &Path, reference_path: &Path) -> Result<MotherSystem, String> {
        Ok(MotherSystem {
            corpus: MotherCorpus::load(corpus_path)?,
            reference: crate::reference::ReferencePack::load(reference_path)?,
        })
    }

    /// Ask a topic question: reference claims with sources plus the
    /// identity she brings to it. Unknown topics return an error naming
    /// the known vocabulary — refusal, never a guess.
    pub fn ask(&self, topic: &str) -> Result<AskOutcome, String> {
        let claims = self.reference.claims_for_topic(topic);
        if claims.is_empty() {
            return Err(format!(
                "no reference for topic {topic:?}; known topics: {}",
                self.reference.topics().join(", ")
            ));
        }
        let claim_lines: Vec<(String, String, String)> = claims
            .iter()
            .map(|claim| {
                let source = self.reference.source_of(claim).expect("resolved at load");
                (
                    claim.statement.clone(),
                    source.title.clone(),
                    source.url.clone(),
                )
            })
            .collect();
        let identity = self
            .corpus
            .query(&Scenario {
                bucket: 9,
                dominance: DOMINANCES
                    .iter()
                    .position(|d| *d == "operational_dominant")
                    .expect("dominance"),
                casts: vec![2], // protectee present
            })
            .expect("identity query");
        Ok(AskOutcome {
            urgent: self.reference.has_urgent(topic),
            claims: claim_lines,
            behaviors: identity.behaviors,
            memories: identity
                .activations
                .iter()
                .take(3)
                .map(|(id, title, _)| (id.clone(), title.clone()))
                .collect(),
            disclaimer: self.reference.disclaimer.clone(),
        })
    }
}
