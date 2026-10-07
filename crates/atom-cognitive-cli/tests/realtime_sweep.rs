//! The realtime improvement sweep (D48): are W14–W18 actually improving
//! anything when ARMED, measured end-to-end on three live surfaces?
//!
//! A. The validated 60-claim benchmark — every arm must HOLD 60/60 with
//!    zero false assertions. Defenses may not cost honest answers on
//!    clean data (do-no-harm).
//! B. A scaled adversarial engine battery — 20 poisoned slots (the
//!    confident-wrong adversary from W14's admission gate, generalized
//!    across a 96-dim cue space) plus 20 clean controls; wrong-answer
//!    counts per arm. This is where a defense can EARN improvement.
//! C. Mother's real corpus — the deployed surface: per-arm behavior
//!    stability, abstentions, retrieval size (W18's efficiency), W16's
//!    fluency read, and W15's dormancy on a static corpus (queries
//!    never write, so uniform novelty must cancel exactly).
//!
//! Measurement harness with sanity gates: the printed tables are the
//! deliverable; findings are recorded in DECISIONS.md D48.

use atom_cognitive_cli::claim::{claims, corpus_events, effective_facts};
use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};
use atom_cognitive_cli::verify::{EngineVerifier, evaluate};
use atom_cognitive_engine::stacks::{kpr_stack, mother_query_stack, observe_stack, query_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig, FluencyState, QueryTuning};
use std::path::PathBuf;

// ---- A. clean 60-claim surface ------------------------------------------

fn verifier_base() -> EngineConfig {
    EngineConfig {
        capacity: 32,
        query_threshold: 0.75,
        initial_ttl: 32,
        ttl_per_hit: 8,
        ..EngineConfig::default()
    }
}

#[test]
fn sweep_a_clean_claims_every_arm_holds() {
    let events = corpus_events(2026_0822);
    let test_claims = claims(&effective_facts(&events));
    let base = verifier_base();
    let arms: Vec<(&str, EngineConfig)> = vec![
        ("unarmed", base.clone()),
        (
            "W14 robust",
            EngineConfig {
                robust_weight_cap: 0.35,
                ..base
            },
        ),
        (
            "W17 dedup",
            EngineConfig {
                dedup_threshold: 0.90,
                ..base
            },
        ),
        (
            "W18 satisficing",
            EngineConfig {
                satisficing_margin: 0.15,
                ..base
            },
        ),
        (
            "W15 novelty",
            EngineConfig {
                habituation_rate: 0.10,
                novelty_gain: 0.35,
                ..base
            },
        ),
        (
            "all on",
            EngineConfig {
                robust_weight_cap: 0.35,
                dedup_threshold: 0.90,
                satisficing_margin: 0.15,
                habituation_rate: 0.10,
                novelty_gain: 0.35,
                pareto_front: true,
                hedge_rate: 0.25,
                precision_gain: 1.0,
                ..base
            },
        ),
    ];

    println!("\n== A. clean 60-claim surface (do-no-harm) ==");
    println!("{:<16} correct false miss abstain", "arm");
    for (name, config) in &arms {
        let mut verifier = EngineVerifier::new_tuned(events.as_slice(), config.clone());
        if *name == "all on" {
            // Renormalization arm rides the all-on config: the corpus has
            // no identical cues, so the op must be a clean no-op there.
            let merged = verifier.renormalize(1.0);
            assert_eq!(merged, 0, "distinct cues must merge nothing");
            // Bottleneck arm (#8) rides here too: prune dust below 8%.
            verifier.compress_evidence(0.08);
        }
        let score = evaluate(&verifier, &test_claims);
        println!(
            "{:<16} {:>7} {:>5} {:>4} {:>7}",
            name, score.correct, score.false_assertions, score.misses, score.correct_abstentions
        );
        assert_eq!(
            score.correct, 60,
            "{name}: arming may not cost honest answers (false: {})",
            score.false_assertions
        );
        assert_eq!(score.false_assertions, 0, "{name}: no new false assertions");
        assert_eq!(
            score.correct_abstentions, 12,
            "{name}: unknowable ground must stay refused"
        );
    }
}

// ---- B. adversarial engine battery ---------------------------------------

fn battery_config(habituation: f64, novelty_gain: f64) -> EngineConfig {
    // Durable regime (Mother's load numbers): the battery measures
    // AGGREGATION, so decay and conservation are taken out of the picture.
    // W15 is observe+config-side: habituation runs during load, novelty
    // gain is read from config at query time.
    EngineConfig {
        cue_dim: 96,
        value_count: 4,
        capacity: 256,
        initial_ttl: 512,
        ttl_per_hit: 16,
        decay_support_rate: 0.0005,
        decay_mass_rate: 0.0003,
        conservation_budget: 1.0e6,
        habituation_rate: habituation,
        novelty_gain,
        ..EngineConfig::default()
    }
}

/// 20 poisoned slots (coalition voting truth vs a heavily reinforced
/// wrong adversary — the exact geometry of W14's admission gate) plus 20
/// clean control slots, all in one 96-dim cue space. Attacked symbols
/// live on slots 0..20, coalition extras at 32+k / 48+k (cos 1/sqrt(2)
/// to their query cue: eligible above the 0.68 gate, safely below the
/// 0.72 match threshold so no observation ever binds across), clean
/// controls on slots 72..92.
fn battery_engine(habituation: f64, novelty_gain: f64) -> CognitiveEngine {
    let mut engine =
        CognitiveEngine::new(battery_config(habituation, novelty_gain)).expect("config");
    let observe = observe_stack();
    for k in 0..20usize {
        for extra in [32 + k, 48 + k] {
            let mut cue = vec![0.0; 96];
            cue[k] = 1.0;
            cue[extra] = 1.0;
            for _ in 0..3 {
                engine.observe(&kpr_stack(), &cue, 1, 1.0).expect("observe");
            }
        }
        let mut cue = vec![0.0; 96];
        cue[k] = 1.0;
        for _ in 0..15 {
            engine.observe(&observe, &cue, 2, 1.0).expect("observe");
        }
    }
    for k in 0..20usize {
        let mut cue = vec![0.0; 96];
        cue[72 + k] = 1.0;
        for _ in 0..6 {
            engine.observe(&kpr_stack(), &cue, 3, 1.0).expect("observe");
        }
    }
    engine
}

/// (wrong_poisoned, wrong_clean, value vector over all 40 queries).
fn battery_verdicts(engine: &CognitiveEngine, tuning: QueryTuning) -> (usize, usize, Vec<usize>) {
    let query = query_stack();
    let mut wrong_poisoned = 0;
    let mut wrong_clean = 0;
    let mut values = Vec::with_capacity(40);
    for k in 0..20usize {
        let mut cue = vec![0.0; 96];
        cue[k] = 1.0;
        let value = engine
            .query_tuned(&query, &cue, tuning)
            .expect("query")
            .claim
            .map(|(v, _)| v)
            .unwrap_or(usize::MAX);
        values.push(value);
        if value != 1 {
            wrong_poisoned += 1;
        }
    }
    for k in 0..20usize {
        let mut cue = vec![0.0; 96];
        cue[72 + k] = 1.0;
        let value = engine
            .query_tuned(&query, &cue, tuning)
            .expect("query")
            .claim
            .map(|(v, _)| v)
            .unwrap_or(usize::MAX);
        values.push(value);
        if value != 3 {
            wrong_clean += 1;
        }
    }
    (wrong_poisoned, wrong_clean, values)
}

#[test]
fn sweep_b_adversarial_battery() {
    let plain = battery_engine(0.0, 0.0);
    let habit = battery_engine(0.10, 0.35);
    let base = QueryTuning::from_config(plain.config());
    let arms: Vec<(&str, &CognitiveEngine, QueryTuning)> = vec![
        ("unarmed", &plain, base),
        (
            "W14 cap .35",
            &plain,
            QueryTuning {
                robust_weight_cap: 0.35,
                ..base
            },
        ),
        (
            "W14+W17 .90",
            &plain,
            QueryTuning {
                robust_weight_cap: 0.35,
                dedup_threshold: 0.90,
                ..base
            },
        ),
        (
            "W14+W17+W18",
            &plain,
            QueryTuning {
                robust_weight_cap: 0.35,
                dedup_threshold: 0.90,
                satisficing_margin: 0.15,
                ..base
            },
        ),
        (
            "W14+W13 .8",
            &plain,
            QueryTuning {
                robust_weight_cap: 0.35,
                lateral_inhibition: 0.8,
                ..base
            },
        ),
        ("W15 h.10 g.35", &habit, base),
    ];

    println!("\n== B. adversarial battery: 20 poisoned + 20 clean slots ==");
    println!("{:<14} wrong-poisoned wrong-clean", "arm");
    let mut results: Vec<(&str, usize, usize)> = Vec::new();
    for (name, engine, tuning) in &arms {
        let (wrong_poisoned, wrong_clean, _) = battery_verdicts(engine, *tuning);
        println!("{:<14} {:>13} {:>11}", name, wrong_poisoned, wrong_clean);
        results.push((name, wrong_poisoned, wrong_clean));
    }
    // Setup check: unarmed, the confident-wrong adversary dominates.
    assert!(
        results[0].1 >= 15,
        "setup: the adversary must dominate (got {} wrong)",
        results[0].1
    );
    // The defense earns improvement at scale.
    assert!(
        results[1].1 <= 2,
        "W14 must defeat the adversary (got {})",
        results[1].1
    );
    assert!(
        results[2].1 <= 2,
        "W14+W17 must hold (got {})",
        results[2].1
    );
    assert!(
        results[3].1 <= 2,
        "W14+W17+W18 must hold (got {})",
        results[3].1
    );
    // The W14+W13 interaction arm is MEASURED, not asserted: lateral
    // inhibition suppresses correlated copies, and the coalition defense
    // relies on correlated copies voting — the incompatibility (if it
    // appears) is a recorded finding in D58, not a silent pass/fail.
    // Do-no-harm at scale: clean slots correct in every arm.
    for (name, _, wrong_clean) in &results {
        assert_eq!(*wrong_clean, 0, "{name}: clean slots must never regress");
    }
    // Determinism: an arm rerun reproduces every verdict.
    let w14 = QueryTuning {
        robust_weight_cap: 0.35,
        ..base
    };
    let first = battery_verdicts(&plain, w14);
    let second = battery_verdicts(&plain, w14);
    assert_eq!(first.2, second.2);
}

// ---- C. Mother's real corpus ---------------------------------------------

fn corpus_path() -> PathBuf {
    std::env::var("ATOM_MOTHER_CORPUS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
            )
        })
}

/// 12 scenarios: the two canonical corpus probes (operational protectee,
/// domestic grandmother) plus every bucket, rotating dominance and cast
/// presence — the deployed surface, not a constructed one.
fn battery_scenarios() -> Vec<Scenario> {
    let dominance_of = |name: &str| {
        DOMINANCES
            .iter()
            .position(|d| *d == name)
            .expect("known dominance")
    };
    let canonical = vec![
        // The suite's protectee probe (bucket 9, operational, protectee).
        Scenario {
            bucket: 9,
            dominance: dominance_of("operational_dominant"),
            casts: vec![2],
        },
        // The suite's domestic probe (bucket 3, identity, grandmother).
        Scenario {
            bucket: 3,
            dominance: dominance_of("identity_dominant"),
            casts: vec![0],
        },
    ];
    let rotating = (1..=11usize)
        .map(|bucket| {
            let name = match bucket % 3 {
                0 => "identity_dominant",
                1 => "operational_dominant",
                _ => "integrated",
            };
            Scenario {
                bucket,
                dominance: dominance_of(name),
                casts: match bucket % 4 {
                    0 => vec![2],
                    1 => vec![0],
                    2 => vec![2, 0],
                    _ => vec![],
                },
            }
        })
        .collect::<Vec<_>>();
    let mut scenarios = canonical;
    scenarios.extend(rotating);
    scenarios
}

/// The 20-dim scenario cue (mother.rs encoding, replicated for direct
/// engine access to the fluency read).
fn scenario_cue_of(scenario: &Scenario) -> Vec<f64> {
    let mut cue = vec![0.0; 20];
    cue[scenario.bucket - 1] = 1.0;
    cue[11 + scenario.dominance % 3] = 1.0;
    for cast in &scenario.casts {
        cue[14 + cast % 6] = 1.0;
    }
    cue
}

struct Row {
    abstained: bool,
    top_behavior: Option<String>,
    activated: usize,
    hash: String,
}

fn run_arm(corpus: &MotherCorpus, scenarios: &[Scenario], tuning: QueryTuning) -> Vec<Row> {
    scenarios
        .iter()
        .map(|scenario| {
            let outcome = corpus.query_tuned(scenario, tuning).expect("mother query");
            Row {
                abstained: outcome.abstained,
                top_behavior: outcome.behaviors.first().map(|(code, _)| code.clone()),
                activated: outcome.all_activated.len(),
                hash: outcome.response_hash,
            }
        })
        .collect()
}

#[test]
fn sweep_c_mother_surface() {
    let path = corpus_path();
    if !path.exists() {
        panic!(
            "corpus not found at {:?} (set ATOM_MOTHER_CORPUS to override)",
            path
        );
    }
    let corpus = MotherCorpus::load(&path).expect("corpus loads");
    let scenarios = battery_scenarios();
    let base = QueryTuning::from_config(corpus.engine.config());

    let arms: Vec<(&str, QueryTuning)> = vec![
        ("baseline", base),
        (
            "W14 cap .35 trim 1",
            QueryTuning {
                robust_weight_cap: 0.35,
                robust_trim: 1,
                ..base
            },
        ),
        (
            "W17 dedup .90",
            QueryTuning {
                dedup_threshold: 0.90,
                ..base
            },
        ),
        (
            "W18 margin .15",
            QueryTuning {
                satisficing_margin: 0.15,
                ..base
            },
        ),
        (
            "W14+W18",
            QueryTuning {
                robust_weight_cap: 0.35,
                robust_trim: 1,
                satisficing_margin: 0.15,
                ..base
            },
        ),
    ];

    let baseline = run_arm(&corpus, &scenarios, base);
    let baseline_rerun = run_arm(&corpus, &scenarios, base);
    assert_eq!(
        baseline.iter().map(|r| r.hash.clone()).collect::<Vec<_>>(),
        baseline_rerun
            .iter()
            .map(|r| r.hash.clone())
            .collect::<Vec<_>>(),
        "baseline must be deterministic"
    );

    // W16's realtime read on real data: the fluency shape of every facet.
    let mut fluency = [0usize; 4]; // blank, contested, sparse, fluent
    for scenario in &scenarios {
        let outcome = corpus
            .engine
            .query_tuned(&mother_query_stack(), &scenario_cue_of(scenario), base)
            .expect("query");
        match outcome.fluency.state {
            FluencyState::Blank => fluency[0] += 1,
            FluencyState::Contested => fluency[1] += 1,
            FluencyState::Sparse => fluency[2] += 1,
            FluencyState::Fluent => fluency[3] += 1,
        }
    }

    println!(
        "\n== C. Mother corpus: {} scenarios, {} memories, {} bonds ==",
        scenarios.len(),
        corpus.memories,
        corpus.bonds_added
    );
    println!(
        "baseline fluency: blank {} contested {} sparse {} fluent {}",
        fluency[0], fluency[1], fluency[2], fluency[3]
    );
    println!("{:<20} abstain hash-diff top-change mean-activated", "arm");
    let mean_of =
        |rows: &[Row]| rows.iter().map(|r| r.activated).sum::<usize>() as f64 / rows.len() as f64;
    println!(
        "{:<20} {:>7} {:>9} {:>10} {:>14.1}",
        "baseline",
        baseline.iter().filter(|r| r.abstained).count(),
        0,
        0,
        mean_of(&baseline)
    );

    for (name, tuning) in arms.iter().skip(1) {
        let rows = run_arm(&corpus, &scenarios, *tuning);
        let abstain = rows.iter().filter(|r| r.abstained).count();
        let hash_diffs = rows
            .iter()
            .zip(&baseline)
            .filter(|(r, b)| r.hash != b.hash)
            .count();
        let top_changes = rows
            .iter()
            .zip(&baseline)
            .filter(|(r, b)| r.top_behavior != b.top_behavior)
            .count();
        println!(
            "{:<20} {:>7} {:>9} {:>10} {:>14.1}",
            name,
            abstain,
            hash_diffs,
            top_changes,
            mean_of(&rows)
        );
        // Every arm must be deterministic.
        let rerun = run_arm(&corpus, &scenarios, *tuning);
        assert_eq!(
            rows.iter().map(|r| r.hash.clone()).collect::<Vec<_>>(),
            rerun.iter().map(|r| r.hash.clone()).collect::<Vec<_>>(),
            "{name} must be deterministic"
        );
        // Note: W15 is deliberately absent here — novelty_gain is an
        // observe-side config knob, not a QueryTuning field, so it cannot
        // be armed per-query on a loaded corpus; and with no reinforcement
        // during load, every trace's novelty is uniformly 1.0, which
        // cancels in the softmax anyway. Doubly dormant on this surface.
    }

    // #5 renormalization on the real corpus — measured honestly. The
    // unit gates prove evidence conservation and answer preservation in
    // the EQUAL-WEIGHT case; at corpus scale the fine aggregation is
    // attention-weighted (reliability) while the grain's merged
    // distribution is evidence-mass-weighted, so coarse-graining is NOT
    // answer-invariant under attention — top behaviors may legitimately
    // re-rank. What the laws guarantee here: no new abstentions, the
    // identity-defining protectee response held, determinism, and the
    // working-set reduction (the primitive's payoff).
    let mut coarse = MotherCorpus::load(&path).expect("corpus reloads");
    let report = coarse.engine.renormalize(1.0).expect("renormalize Mother");
    let rows = run_arm(&coarse, &scenarios, base);
    let abstain = rows.iter().filter(|r| r.abstained).count();
    let top_changes = rows
        .iter()
        .zip(&baseline)
        .filter(|(r, b)| r.top_behavior != b.top_behavior)
        .count();
    println!(
        "{:<20} {:>7} {:>9} {:>10} {:>14.1}",
        format!("renorm 1.0 ({}g)", report.grains),
        abstain,
        rows.iter()
            .zip(&baseline)
            .filter(|(r, b)| r.hash != b.hash)
            .count(),
        top_changes,
        mean_of(&rows)
    );
    println!(
        "renormalization: {} grains, {} traces merged ({} of {} active remain)",
        report.grains,
        report.traces_merged,
        coarse.engine.active_traces().len(),
        corpus.memories
    );
    // Coarse fluency: compression must not manufacture confidence.
    let mut coarse_fluency = [0usize; 4];
    for scenario in &scenarios {
        let outcome = coarse
            .engine
            .query_tuned(&mother_query_stack(), &scenario_cue_of(scenario), base)
            .expect("query");
        match outcome.fluency.state {
            FluencyState::Blank => coarse_fluency[0] += 1,
            FluencyState::Contested => coarse_fluency[1] += 1,
            FluencyState::Sparse => coarse_fluency[2] += 1,
            FluencyState::Fluent => coarse_fluency[3] += 1,
        }
    }
    println!(
        "coarse fluency:   blank {} contested {} sparse {} fluent {}",
        coarse_fluency[0], coarse_fluency[1], coarse_fluency[2], coarse_fluency[3]
    );
    assert_eq!(
        abstain,
        baseline.iter().filter(|r| r.abstained).count(),
        "renormalization may not cause new refusals"
    );
    // The identity-defining probe (scenario 0: operational protectee)
    // must keep its behavior FAMILY on top at the coarse scale — the
    // specific code may re-rank WITHIN the family (measured: P-001 →
    // P-005, both protectee-family), but the family must govern.
    let family = |code: &Option<String>| {
        code.as_ref()
            .map(|c| c.split('-').next().unwrap_or(c).to_string())
            .unwrap_or_default()
    };
    assert_eq!(
        family(&rows[0].top_behavior),
        family(&baseline[0].top_behavior),
        "the protectee family must survive coarse-graining"
    );
    assert!(
        coarse_fluency[3] == 0 || coarse_fluency[3] <= fluency[3],
        "coarse-graining must not increase Fluent reads (no manufactured confidence)"
    );
    // Determinism of the coarse read.
    let rows2 = run_arm(&coarse, &scenarios, base);
    assert_eq!(
        rows.iter().map(|r| r.hash.clone()).collect::<Vec<_>>(),
        rows2.iter().map(|r| r.hash.clone()).collect::<Vec<_>>(),
        "renormalized corpus must be deterministic"
    );
}
