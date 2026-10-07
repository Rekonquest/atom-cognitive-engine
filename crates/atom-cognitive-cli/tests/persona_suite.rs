//! Persona-governor gates (D33/D34): mode tracking, restraint under
//! pressure, the no-mutation authority boundary, and floor enforcement.

use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};
use atom_cognitive_cli::persona::{PersonaGovernor, PersonaMode};
use atom_cognitive_engine::QueryTuning;
use std::path::PathBuf;

fn corpus() -> MotherCorpus {
    let path = std::env::var("ATOM_MOTHER_CORPUS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
            )
        });
    MotherCorpus::load(&path).expect("corpus loads")
}

fn dominance_index(name: &str) -> usize {
    DOMINANCES
        .iter()
        .position(|d| *d == name)
        .expect("known dominance")
}

#[test]
fn gate_mode_tracking() {
    let corpus = corpus();
    let mut governor = PersonaGovernor::new();
    // Data-grounded expectations (verified against the approved corpus):
    // every approved bucket-9 protectee-present memory is INTEGRATED
    // dominance — protectee situations activate her mother mode, not the
    // pure operator. The kitchen scenario is identity-dominant (civilian).
    let outcome = corpus
        .query(&Scenario {
            bucket: 9,
            dominance: dominance_index("operational_dominant"),
            casts: vec![2],
        })
        .expect("query");
    governor.update(&corpus, &outcome, 0.0);
    assert_eq!(governor.mode(), PersonaMode::Integrated);
    // A childhood kitchen scenario drives the civilian mode — and the mode
    // must CHANGE between situations (the tracking mechanism works).
    let outcome = corpus
        .query(&Scenario {
            bucket: 3,
            dominance: dominance_index("identity_dominant"),
            casts: vec![0],
        })
        .expect("query");
    governor.update(&corpus, &outcome, 0.0);
    assert_eq!(governor.mode(), PersonaMode::Civilian);
}

#[test]
fn gate_restraint_under_pressure() {
    // SG-004 as an engine behavior: high arousal must produce MORE
    // abstentions and NEVER a new wrong activation. Measured on corrupted
    // (unknowable-ish) ground: the governed engine abstains where the
    // ungoverned one may still assert.
    let corpus = corpus();
    let base = QueryTuning::from_config(corpus.engine.config());
    let mut governor = PersonaGovernor::new();
    for _ in 0..3 {
        governor.update(
            &corpus,
            &corpus
                .query(&Scenario {
                    bucket: 9,
                    dominance: dominance_index("operational_dominant"),
                    casts: vec![2],
                })
                .expect("query"),
            0.9,
        );
    }
    assert!(governor.arousal() > 0.6, "pressure must accumulate");
    let decision = governor.decision(&base);
    // The restraint citations must be present.
    assert!(decision.citations.iter().any(|(code, _)| code == "SG-004"));
    assert!(decision.citations.iter().any(|(code, _)| code == "SG-001"));
    // The gate must be STRICTER than baseline, never looser.
    assert!(
        decision.tuning.query_threshold > base.query_threshold,
        "high arousal must tighten the gate"
    );
    assert!(decision.tuning.min_reliability >= base.min_reliability);

    // Behavioral consequence: on corrupted ground the governed engine
    // abstains at least as often as the ungoverned one, and never asserts
    // something the ungoverned engine abstained on... strictly: the
    // governed engine's eligible set is a subset.
    let stack = atom_cognitive_engine::stacks::mother_query_stack();
    let mut governed_abstentions = 0usize;
    let mut ungoverned_abstentions = 0usize;
    for casts in [vec![2usize, 5, 4], vec![2, 5], vec![2, 0, 4]] {
        let scenario = Scenario {
            bucket: 9,
            dominance: dominance_index("operational_dominant"),
            casts,
        };
        let cue = scenario_cue(&scenario);
        let plain = corpus.engine.query(&stack, &cue).expect("plain");
        let tuned = corpus
            .engine
            .query_tuned(&stack, &cue, decision.tuning)
            .expect("tuned");
        if plain.claim.is_some() {
            ungoverned_abstentions += 0;
        } else {
            ungoverned_abstentions += 1;
        }
        if tuned.claim.is_some() {
            governed_abstentions += 0;
        } else {
            governed_abstentions += 1;
        }
        // The eligible set under stricter tuning is a subset.
        assert!(tuned.eligible_count <= plain.eligible_count);
    }
    assert!(
        governed_abstentions >= ungoverned_abstentions,
        "the governed engine must not assert MORE under pressure"
    );
}

fn scenario_cue(scenario: &Scenario) -> Vec<f64> {
    let mut cue = vec![0.0; 20];
    cue[scenario.bucket - 1] = 1.0;
    cue[11 + scenario.dominance % 3] = 1.0;
    for cast in &scenario.casts {
        cue[14 + cast % 6] = 1.0;
    }
    cue
}

#[test]
fn gate_no_mutation_authority() {
    // The governor may tune knobs; it must NEVER change engine state.
    let corpus = corpus();
    let before = corpus.engine.traces().to_vec();
    let base = QueryTuning::from_config(corpus.engine.config());
    let mut governor = PersonaGovernor::new();
    let outcome = corpus
        .query(&Scenario {
            bucket: 9,
            dominance: dominance_index("operational_dominant"),
            casts: vec![2],
        })
        .expect("query");
    governor.update(&corpus, &outcome, 1.0);
    let decision = governor.decision(&base);
    let stack = atom_cognitive_engine::stacks::mother_query_stack();
    let cue = scenario_cue(&Scenario {
        bucket: 9,
        dominance: dominance_index("operational_dominant"),
        casts: vec![2],
    });
    let _ = corpus
        .engine
        .query_tuned(&stack, &cue, decision.tuning)
        .expect("tuned query");
    let after = corpus.engine.traces().to_vec();
    assert_eq!(
        before, after,
        "a governed query must not mutate a single trace"
    );
    assert_eq!(
        corpus.engine.bond_count(),
        120,
        "bonds untouched by the governor"
    );
}

#[test]
fn gate_floors_are_absolute() {
    // The governor cannot dismantle the gate: try to prescribe an extreme
    // loosening through a corrupted decision path and verify clamping.
    let base = QueryTuning {
        attention_temperature: 2.5,
        query_threshold: 0.60,
        min_reliability: 0.16,
        bond_expansion_depth: 2,
        robust_weight_cap: 1.0,
        robust_trim: 0,
        dedup_threshold: 1.01,
        satisficing_margin: 0.0,
        pareto_front: false,
        stigmergy_rate: 0.0,
        stigmergy_gain: 0.0,
        lateral_inhibition: 0.0,
        quorum_min: 0,
    };
    let dangerous = QueryTuning {
        attention_temperature: 0.0,
        query_threshold: 0.0,
        min_reliability: 0.0,
        bond_expansion_depth: 99,
        robust_weight_cap: 0.0,
        robust_trim: 99,
        dedup_threshold: 0.1,
        satisficing_margin: 2.0,
        pareto_front: true,
        stigmergy_rate: 1.0,
        stigmergy_gain: 1.0,
        lateral_inhibition: 1.0,
        quorum_min: 64,
    };
    let clamped = dangerous.clamped();
    assert!(clamped.attention_temperature >= 0.5);
    assert!(clamped.query_threshold >= 0.5);
    assert!(clamped.min_reliability >= 0.05);
    // Ceiling raised 4 -> 8 (2026-08-30, operator: "increase the hops"),
    // gated by tests/bonds.rs depth-8 chain crossings.
    assert!(clamped.bond_expansion_depth <= 8);
    assert!(clamped.robust_weight_cap >= 0.15);
    assert!(clamped.robust_trim <= 8);
    assert!(clamped.dedup_threshold >= 0.5);
    assert!(clamped.satisficing_margin <= 1.0);
    // The governor's own floors sit above the engine's hard floors.
    let corpus = corpus();
    let mut governor = PersonaGovernor::new();
    let outcome = corpus
        .query(&Scenario {
            bucket: 3,
            dominance: dominance_index("identity_dominant"),
            casts: vec![0],
        })
        .expect("query");
    governor.update(&corpus, &outcome, 0.0);
    let decision = governor.decision(&base);
    assert!(decision.tuning.query_threshold >= governor.floor_threshold);
    assert!(decision.tuning.min_reliability >= governor.floor_reliability);
    // Every decision carries citations.
    assert!(!decision.citations.is_empty());
}
