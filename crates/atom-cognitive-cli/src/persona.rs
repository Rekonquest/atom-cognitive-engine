//! The persona governor (D33): Mother as an ACTIVE component, not just
//! content. Her state — which mode is dominant, how much pressure she is
//! under — modulates HOW the engine attends and gates on the next query.
//!
//! The authority boundary is the whole design: the governor controls
//! attention temperature, gate strictness, reliability floors, and echo
//! depth. It can NEVER mutate traces, evidence, tiers, bonds, or the
//! engine config; every tuning is clamped to safety floors; every delta
//! carries a citation (the behaviors and memories that drove it). She
//! shapes the attention, never the truth.
//!
//! Behavioral grounding (from the corpus's own spec): SG-004 (recognize
//! dysregulation) and SG-001 (don't spend rage impulsively) → under high
//! arousal the governor RAISES the gate threshold and pulls echo depth
//! back: assert less, abstain more — restraint as an engine behavior.
//! P-002 (distinguish immediate vs plan-required) with operational mode →
//! sharpen attention. Civilian mode with low arousal → warmer, slightly
//! more receptive attention, floored.

use crate::mother::{MotherCorpus, ScenarioOutcome};
use atom_cognitive_engine::QueryTuning;

/// Current persona mode, derived from the dominance of activated memories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaMode {
    Civilian,
    Operational,
    Integrated,
}

/// One governor decision: the tuning for the next query plus the citations
/// that justify each change.
#[derive(Debug, Clone, PartialEq)]
pub struct GovernorDecision {
    pub mode: PersonaMode,
    /// 0.0 (calm) .. 1.0 (highly pressurized).
    pub arousal: f64,
    pub tuning: QueryTuning,
    /// (behavior code, what it changed) — the receipts.
    pub citations: Vec<(String, String)>,
}

pub struct PersonaGovernor {
    mode: PersonaMode,
    arousal: f64,
    /// Safety floors: the governor can never loosen the gate below this.
    pub floor_threshold: f64,
    pub floor_reliability: f64,
}

impl PersonaGovernor {
    pub fn new() -> PersonaGovernor {
        PersonaGovernor {
            mode: PersonaMode::Integrated,
            arousal: 0.0,
            floor_threshold: 0.60,
            floor_reliability: 0.16,
        }
    }

    pub fn mode(&self) -> PersonaMode {
        self.mode
    }

    pub fn arousal(&self) -> f64 {
        self.arousal
    }

    /// Update persona state from the last scenario outcome plus a pressure
    /// signal (0.0 calm .. 1.0 adversarial). Mode follows the majority
    /// dominance among the top activated memories; arousal rises with
    /// pressure and decays by half each cycle.
    pub fn update(&mut self, corpus: &MotherCorpus, outcome: &ScenarioOutcome, pressure: f64) {
        // Mode: majority dominance among the top-5 cited activations.
        let mut counts = [0usize; 3]; // identity, operational, integrated
        for (memory_id, _, _) in outcome.activations.iter().take(5) {
            if let Some(trace) = corpus
                .trace_id_of(memory_id)
                .and_then(|trace_id| corpus.engine.trace_by_id(trace_id))
            {
                // Dominance lives at cue dims 11..14 (set at load).
                for (offset, count) in counts.iter_mut().enumerate() {
                    if trace.cue.get(11 + offset).is_some_and(|v| *v > 0.3) {
                        *count += 1;
                    }
                }
            }
        }
        if counts.iter().any(|count| *count > 0) {
            let best = counts
                .iter()
                .enumerate()
                .max_by_key(|(_, count)| **count)
                .map(|(index, _)| index)
                .unwrap_or(2);
            self.mode = match best {
                0 => PersonaMode::Civilian,
                1 => PersonaMode::Operational,
                _ => PersonaMode::Integrated,
            }
        }
        // Arousal: half-life decay, pressure raises it, bounded.
        let pressure = pressure.clamp(0.0, 1.0);
        self.arousal = ((self.arousal * 0.5) + pressure).clamp(0.0, 1.0);
    }

    /// The tuning the persona currently prescribes, with citations.
    pub fn decision(&self, base: &QueryTuning) -> GovernorDecision {
        let mut tuning = *base;
        let mut citations: Vec<(String, String)> = Vec::new();
        match self.mode {
            PersonaMode::Operational => {
                // P-002: sharpen attention in operational mode.
                tuning.attention_temperature = (base.attention_temperature + 0.5).min(6.0);
                tuning.query_threshold = (base.query_threshold + 0.05).max(self.floor_threshold);
                citations.push((
                    "P-002".into(),
                    "operational mode: attention sharpened, gate tightened +0.05".into(),
                ));
            }
            PersonaMode::Civilian => {
                // Civilian default: warmer, slightly more receptive.
                tuning.attention_temperature = (base.attention_temperature + 0.5).min(6.0);
                tuning.query_threshold = (base.query_threshold - 0.03).max(self.floor_threshold);
                citations.push((
                    "C-001".into(),
                    "civilian default: warmer attention, gate loosened 0.03 (floored)".into(),
                ));
            }
            PersonaMode::Integrated => {
                citations.push((
                    "R-005".into(),
                    "integrated mode: baseline attention maintained".into(),
                ));
            }
        }
        if self.arousal > 0.6 {
            // SG-004 + SG-001: under dysregulation, assert less.
            tuning.query_threshold =
                (tuning.query_threshold + 0.10).clamp(self.floor_threshold, 0.999);
            tuning.min_reliability = (tuning.min_reliability + 0.10).max(self.floor_reliability);
            tuning.bond_expansion_depth = tuning.bond_expansion_depth.saturating_sub(1);
            citations.push((
                "SG-004".into(),
                format!(
                    "high arousal ({:.2}): gate +0.10, reliability +0.10, echo depth pulled back — assert less",
                    self.arousal
                ),
            ));
            citations.push((
                "SG-001".into(),
                "restraint under pressure: do not spend certainty impulsively".into(),
            ));
        }
        // Floors are absolute: the governor can never dismantle the gate.
        tuning.query_threshold = tuning.query_threshold.max(self.floor_threshold);
        tuning.min_reliability = tuning.min_reliability.max(self.floor_reliability);
        GovernorDecision {
            mode: self.mode,
            arousal: self.arousal,
            tuning: tuning.clamped(),
            citations,
        }
    }
}

impl Default for PersonaGovernor {
    fn default() -> Self {
        Self::new()
    }
}
