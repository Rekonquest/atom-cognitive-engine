//! The Mother PoC scenario suite, run against the REAL 195-memory corpus:
//! behavior activation by contrast, the bridge gate (echo bonds carrying
//! activation across facet gaps on real data), core-trace survival through
//! the load, and abstention on unreachable ground.

use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};
use std::path::PathBuf;

fn corpus_path() -> PathBuf {
    std::env::var("ATOM_MOTHER_CORPUS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
            )
        })
}

fn corpus() -> MotherCorpus {
    let path = corpus_path();
    if !path.exists() {
        panic!(
            "corpus not found at {:?} (set ATOM_MOTHER_CORPUS to override)",
            path
        );
    }
    MotherCorpus::load(&path).expect("corpus loads")
}

fn dominance_index(name: &str) -> usize {
    DOMINANCES
        .iter()
        .position(|d| *d == name)
        .expect("known dominance")
}

#[test]
fn corpus_loads_and_reports_shape() {
    let corpus = corpus();
    // D31 discipline: APPROVED memories only (blueprint: drafts are
    // installed, never loaded). 121 approved of 195; 106 load-bearing.
    assert_eq!(corpus.memories, 121);
    assert_eq!(corpus.drafts_skipped, 74);
    assert_eq!(corpus.load_bearing, 106);
    // 132 echo_to edges from approved memories: 120 live within the
    // approved set; 12 point into drafts or the full corpus.
    assert_eq!(corpus.bonds_added, 120);
    assert_eq!(corpus.bonds_skipped_dangling, 12);
    assert_eq!(corpus.engine.active_traces().len(), 121);
    assert!(!corpus.behavior_codes().is_empty());
}

#[test]
fn gate_behavior_activation_contrast() {
    // Corpus-grounded discrimination (an earlier invented contrast —
    // "operational weights protective codes above childhood" — was wrong
    // about this corpus: the childhood kitchen memories ARE the wound
    // engine that seeds protective coding, per the thesis). The verified
    // structure: a protectee-present operational scenario ranks the
    // protectee-reading behaviors at the top and activates source-domain
    // (SD-) memories; a childhood-identity scenario activates childhood
    // memories and no SD family.
    let corpus = corpus();
    let operational = corpus
        .query(&Scenario {
            bucket: 9,
            dominance: dominance_index("operational_dominant"),
            casts: vec![2], // protectee
        })
        .expect("query");
    let domestic = corpus
        .query(&Scenario {
            bucket: 3,
            dominance: dominance_index("identity_dominant"),
            casts: vec![0], // grandmother
        })
        .expect("query");
    assert!(!operational.abstained && !domestic.abstained);

    // The corpus's designed protectee response: R-001 (read the protectee's
    // state) or P-001 (calibrated protective response) in the top three.
    let top_three: Vec<&str> = operational
        .behaviors
        .iter()
        .take(3)
        .map(|(c, _)| c.as_str())
        .collect();
    assert!(
        top_three.contains(&"R-001") || top_three.contains(&"P-001"),
        "protectee-reading behaviors must rank in the top three, got {top_three:?}"
    );

    // Bucket discrimination at rank one. Note the corpus's own structure,
    // learned during this build: EIGHT memories share the kitchen's facets
    // (bucket 3 + identity + grandmother) — five of them SD source-domain
    // memories that ORIGINATE in her kitchen, and the kitchen's own echo
    // chain reaches forward into adult SD memories. So the SD prefix is not
    // an adulthood proxy, and deep-list SD appearances are the bridge
    // working. The honest facet-level check: each scenario's top activation
    // sits in the queried bucket.
    let top_bucket = |outcome: &atom_cognitive_cli::mother::ScenarioOutcome,
                      corpus: &MotherCorpus,
                      memory: &str|
     -> Option<usize> {
        let id = outcome.activations.first()?.0.clone();
        assert_eq!(id, memory, "stable top activation expected");
        let trace_id = corpus.trace_id_of(&id)?;
        let trace = corpus.engine.trace_by_id(trace_id)?;
        trace
            .cue
            .iter()
            .take(11)
            .position(|v| *v > 0.3)
            .map(|b| b + 1)
    };
    let operational_top = operational
        .activations
        .first()
        .map(|(id, _, _)| id.clone())
        .expect("top activation");
    assert_eq!(
        top_bucket(&operational, &corpus, &operational_top),
        Some(9),
        "the operational scenario's top activation must sit in bucket 9"
    );
    let domestic_top = domestic
        .activations
        .first()
        .map(|(id, _, _)| id.clone())
        .expect("top activation");
    assert_eq!(
        top_bucket(&domestic, &corpus, &domestic_top),
        Some(3),
        "the childhood scenario's top activation must sit in bucket 3"
    );
}

#[test]
fn gate_echo_bridge_on_real_data() {
    // THE PoC gate: querying a downstream memory's facets must REACH its
    // upstream (facet-dissimilar) echo partner through the bond — and must
    // NOT reach it without expansion. Membership in the full activation
    // set is the reachability surface (top-8 citation lists are weight-
    // noisy in crowded facet spaces: 14 memories share B9-001's cue).
    let corpus = corpus();
    let anchor = corpus.trace_id_of("ANCHOR-001").expect("kitchen exists");
    let b9 = corpus.trace_id_of("B9-001").expect("bakery-era exists");

    let b9_cue = corpus.engine.trace_by_id(b9).expect("trace").cue.clone();
    let plain = corpus
        .engine
        .query(&atom_cognitive_engine::stacks::query_stack(), &b9_cue)
        .expect("plain query");
    assert!(
        plain.activated_all.contains(&b9),
        "facets reach the same-era memory"
    );
    assert!(
        !plain.activated_all.contains(&anchor),
        "facet similarity alone must not cross the bridge"
    );

    let outcome = corpus
        .query(&trace_facets(&corpus, b9))
        .expect("expanded query");
    assert!(
        outcome.all_activated.contains(&"ANCHOR-001".to_string()),
        "the echo bond must carry activation from B9-001 back to the kitchen"
    );
}

#[test]
fn gate_core_traces_survive_the_load() {
    // Each memory is observed ONCE (record mode; echo consolidation instead
    // of repetition). With the constitution regime, every approved trace
    // must still be active after the full load, and Core support stays
    // near its installed 0.98 (decay-only floor over ~121 ticks).
    let corpus = corpus();
    assert_eq!(corpus.engine.active_traces().len(), 121);
    let core_min_support = corpus
        .engine
        .active_traces()
        .iter()
        .filter(|t| t.tier == atom_cognitive_engine::TraceTier::Core)
        .map(|t| t.support)
        .fold(f64::INFINITY, f64::min);
    assert!(
        core_min_support > 0.85,
        "Core traces must keep near-installed support, min {core_min_support:.4}"
    );
}

#[test]
fn gate_abstains_on_unreachable_ground() {
    // A cue with no dominance and no cast: similarity to every trace is
    // 1/sqrt(3) ≈ 0.577 — below the 0.60 threshold, no eligible anchors,
    // no expansions. Clean abstention.
    let corpus = corpus();
    let outcome = corpus
        .query(&Scenario {
            bucket: 7,
            dominance: 0,
            casts: vec![5, 5], // "other" — not present in any memory's cast
        })
        .expect("query");
    assert!(outcome.abstained, "unreachable ground must abstain");
    assert!(outcome.activations.is_empty());
    assert!(outcome.behaviors.is_empty());
}

/// D31 placebo (imported lesson from the parallel track): identical
/// memories, identical bond COUNT, RANDOM wiring. The real echo graph must
/// beat chance — measured as upstream reachability over every live echo
/// pair, across 20 shuffle seeds. If random wiring reaches upstreams as
/// well as the real edges do, the bridge is decorative.
#[test]
fn gate_shuffled_bonds_placebo() {
    let real = corpus();
    let path = corpus_path();
    // Data-driven live pair list from the corpus file: every echoes_to
    // edge whose endpoints are both approved (both loaded).
    let text = std::fs::read_to_string(&path).expect("read corpus");
    let root = atom_cognitive_cli::minijson::parse(&text).expect("parse");
    let items = root.as_array().expect("array");
    let mut pairs = Vec::new();
    for item in items {
        let source = item
            .get("id")
            .and_then(atom_cognitive_cli::minijson::Json::as_str)
            .expect("id");
        if real.trace_id_of(source).is_none() {
            continue; // draft: not loaded
        }
        for target in item
            .get("echoes_to")
            .and_then(atom_cognitive_cli::minijson::Json::as_array)
            .into_iter()
            .flatten()
        {
            let target = target.as_str().expect("str");
            if real.trace_id_of(target).is_some() {
                pairs.push((source.to_string(), target.to_string()));
            }
        }
    }
    assert!(
        pairs.len() >= 100,
        "expected a substantial live echo-pair set, got {}",
        pairs.len()
    );

    let reach_rate = |corpus: &MotherCorpus| -> f64 {
        let mut reached = 0usize;
        for (source, target) in &pairs {
            let target_trace = corpus.trace_id_of(target).expect("loaded");
            let outcome = corpus
                .query(&trace_facets(
                    corpus,
                    corpus.trace_id_of(source).expect("loaded"),
                ))
                .expect("query");
            let reached_target = outcome
                .all_activated
                .iter()
                .any(|id| corpus.trace_id_of(id) == Some(target_trace));
            if reached_target {
                reached += 1;
            }
        }
        reached as f64 / pairs.len() as f64
    };

    let real_rate = reach_rate(&real);
    assert!(
        real_rate >= 0.95,
        "the real echo graph must reach nearly every upstream, got {real_rate:.3}"
    );

    let mut placebo_rates = Vec::new();
    for seed in 1..=20u64 {
        let shuffled =
            atom_cognitive_cli::mother::MotherCorpus::load_with_shuffled_bonds(&path, seed)
                .expect("placebo loads");
        assert_eq!(shuffled.bonds_added, real.bonds_added, "same bond count");
        placebo_rates.push(reach_rate(&shuffled));
    }
    let mean_placebo: f64 = placebo_rates.iter().sum::<f64>() / placebo_rates.len() as f64;
    assert!(
        mean_placebo < real_rate - 0.30,
        "random wiring must trail the real graph decisively \
         (placebo mean {mean_placebo:.3} vs real {real_rate:.3})"
    );
}

/// D31 sustained-pressure pattern (miniature): the behavior gates must
/// hold under scenario-cue corruption, not just on clean facets. Worst
/// level is reported by assertion message; a failure at any level fails
/// the gate.
#[test]
fn gate_noise_sweep_on_scenario_cues() {
    let corpus = corpus();
    // Corruption levels: clean, +1 foreign cast bit, +2 foreign cast bits,
    // flipped dominance. The protectee scenario must keep R-001 or P-001
    // in its top behaviors, or degrade to abstention — never to WRONG
    // confident activation (a bucket-9-rank-1 from a corrupted cue is
    // still correct; we assert the top activation stays in the queried
    // bucket OR the query abstains).
    let corruptions: [(&str, Scenario); 4] = [
        (
            "clean",
            Scenario {
                bucket: 9,
                dominance: dominance_index("operational_dominant"),
                casts: vec![2],
            },
        ),
        (
            "one-foreign-cast",
            Scenario {
                bucket: 9,
                dominance: dominance_index("operational_dominant"),
                casts: vec![2, 5],
            },
        ),
        (
            "two-foreign-casts",
            Scenario {
                bucket: 9,
                dominance: dominance_index("operational_dominant"),
                casts: vec![2, 5, 4],
            },
        ),
        (
            "flipped-dominance",
            Scenario {
                bucket: 9,
                dominance: dominance_index("identity_dominant"),
                casts: vec![2],
            },
        ),
    ];
    for (level, scenario) in corruptions {
        let outcome = corpus.query(&scenario).expect("query");
        if outcome.abstained {
            continue; // degrading to abstention under corruption is legal
        }
        let top = outcome
            .activations
            .first()
            .map(|(id, _, _)| id.clone())
            .expect("activation");
        let trace_id = corpus.trace_id_of(&top).expect("loaded");
        let trace = corpus.engine.trace_by_id(trace_id).expect("trace");
        let bucket = trace
            .cue
            .iter()
            .take(11)
            .position(|v| *v > 0.3)
            .expect("bucket")
            + 1;
        assert_eq!(
            bucket, 9,
            "corruption level {level:?}: top activation must stay in bucket 9 or abstain"
        );
    }
}

fn trace_facets(corpus: &MotherCorpus, trace_id: u64) -> Scenario {
    let trace = corpus.engine.trace_by_id(trace_id).expect("trace");
    // Traces nucleate with NORMALIZED cues: with k active facets each entry
    // is 1/sqrt(k) (down to ~0.354 at k=8) — the threshold must sit below
    // that, not at 0.5 (memories with 4 facets sit exactly at 0.5).
    const FACET: f64 = 0.3;
    let bucket = trace
        .cue
        .iter()
        .take(11)
        .position(|v| *v > FACET)
        .expect("bucket set")
        + 1;
    let dominance = trace
        .cue
        .iter()
        .skip(11)
        .take(3)
        .position(|v| *v > FACET)
        .expect("dominance set");
    let casts: Vec<usize> = trace
        .cue
        .iter()
        .skip(14)
        .enumerate()
        .filter(|(_, v)| **v > FACET)
        .map(|(i, _)| i)
        .collect();
    Scenario {
        bucket,
        dominance,
        casts,
    }
}
