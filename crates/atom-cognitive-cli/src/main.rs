//! atom-cognitive-cli — the engine surface.
//!
//! `--self-test` runs the canonical cycles over a deterministic program.
//! `--evaluate` runs the Phase 2 claim-verification benchmark: the engine
//! and three baselines over the same 60-claim three-label test set.
//! `--demo` verifies one claim of each kind with citations.
//! `--mother-demo` loads Mother's real approved-memory constitution (121
//! of 195; drafts are installed but never loaded) into the engine and
//! answers scenario queries with cited activations.
//! `--ask <topic>` answers a topic question through both lanes: sourced
//! reference facts with citations, plus the identity she brings to it.
//! `--persona-demo` shows Mother IN CONTROL: her mode and arousal drive
//! the engine's attention and gating, with citations and floors.

use atom_cognitive_cli::claim::{Label, claims, corpus_events, effective_facts};
use atom_cognitive_cli::verify::{
    ClaimVerifier, EngineVerifier, ExactAddressVerifier, NearestVerifier, SystemScore, Verdict,
    evaluate,
};
use atom_cognitive_engine::stacks::{idle_stack, observe_stack, query_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig};

fn cue(symbol: usize, context: usize) -> Vec<f64> {
    let mut cue = vec![0.0; 12];
    cue[symbol % 8] = 1.0;
    cue[8 + context % 4] = 1.0;
    cue
}

fn run_self_test() -> i32 {
    let observe = observe_stack();
    let query = query_stack();
    let idle = idle_stack();
    let mut engine = CognitiveEngine::new(EngineConfig::default()).expect("config valid");

    for symbol in 0..4 {
        for _ in 0..3 {
            engine
                .observe(&observe, &cue(symbol, 0), symbol, 1.0)
                .expect("observe");
        }
    }
    let revisions: usize = (0..3)
        .filter(|_| {
            engine
                .observe(&observe, &cue(0, 0), 3, 0.25)
                .expect("observe")
                .action
                == "revise"
        })
        .count();
    engine
        .observe(&observe, &cue(5, 2), 2, 0.25)
        .expect("observe");
    engine.idle(&idle, 9).expect("idle");

    let learned: Vec<Option<usize>> = (0..4)
        .map(|symbol| {
            engine
                .query(&query, &cue(symbol, 0))
                .expect("query")
                .claim
                .map(|(v, _)| v)
        })
        .collect();
    let unknown_abstained = engine
        .query(&query, &cue(7, 3))
        .expect("query")
        .claim
        .is_none();

    let gates = [
        // Symbol 0 was deliberately revised to value 3, so the learning
        // gate covers the untouched symbols only.
        (
            "learns_associations",
            learned[1..]
                .iter()
                .enumerate()
                .all(|(i, v)| *v == Some(i + 1)),
        ),
        (
            "revises_contradictions",
            learned[0] == Some(3) && revisions == 3,
        ),
        ("forgets_unsupported", engine.active_traces().len() == 4),
        ("abstains_on_unknown", unknown_abstained),
        (
            "conservation_holds",
            engine.total_mass() <= engine.config().conservation_budget + 1e-9,
        ),
        (
            "explore_cooled",
            engine.explore_temperature() < engine.config().explore_initial,
        ),
    ];
    let passed = gates.iter().all(|(_, ok)| *ok);

    println!("{{");
    println!("  \"runtime\": \"atom-cognitive-engine-v0.1\",");
    println!("  \"observations\": {},", engine.observations());
    println!("  \"surprises\": {},", engine.surprise_count());
    println!("  \"forgotten\": {},", engine.forgotten_count());
    println!("  \"actions\": {{");
    println!("    \"reinforce\": {},", engine.action_count("reinforce"));
    println!("    \"revise\": {},", engine.action_count("revise"));
    println!("    \"nucleate\": {},", engine.action_count("nucleate"));
    println!(
        "    \"capacity_retire\": {},",
        engine.action_count("capacity_retire")
    );
    println!("  }},");
    println!("  \"learned\": {:?},", learned);
    println!(
        "  \"explore_temperature\": {:.4},",
        engine.explore_temperature()
    );
    println!("  \"gates\": {{");
    for (name, ok) in &gates {
        println!("    \"{name}\": {ok},");
    }
    println!("  }},");
    println!("  \"passed\": {passed}");
    println!("}}");
    if passed { 0 } else { 1 }
}

fn run_evaluate() -> i32 {
    let events = corpus_events(2026_0822);
    let test_claims = claims(&effective_facts(&events));
    let engine = EngineVerifier::new(&events);
    let exact = ExactAddressVerifier::new(&events);
    let nearest_always = NearestVerifier::new(&events, false);
    let nearest_thresholded = NearestVerifier::new(&events, true);

    let systems: Vec<SystemScore> = vec![
        evaluate(&engine, &test_claims),
        evaluate(&exact, &test_claims),
        evaluate(&nearest_always, &test_claims),
        evaluate(&nearest_thresholded, &test_claims),
    ];
    let criterion = systems[0].false_assertions == 0
        && systems[0].correct >= systems.iter().map(|s| s.correct).max().unwrap();

    println!("{{");
    println!("  \"runtime\": \"atom-claim-verifier-v1\",");
    println!("  \"claims\": {},", test_claims.len());
    println!("  \"engine_state_hash\": \"{}\",", engine.state_hash());
    println!("  \"systems\": [");
    for system in &systems {
        println!("    {{");
        println!("      \"name\": \"{}\",", system.name);
        println!("      \"correct\": {},", system.correct);
        println!("      \"false_assertions\": {},", system.false_assertions);
        println!("      \"coverage_misses\": {},", system.misses);
        println!(
            "      \"correct_abstentions\": {}",
            system.correct_abstentions
        );
        println!("    }},");
    }
    println!("  ],");
    println!(
        "  \"pinned_criterion\": \"zero false assertions while matching or beating every baseline on correct answers\","
    );
    println!("  \"criterion_met\": {criterion}");
    println!("}}");
    if criterion { 0 } else { 1 }
}

fn run_demo() -> i32 {
    let events = corpus_events(2026_0822);
    let test_claims = claims(&effective_facts(&events));
    let engine = EngineVerifier::new(&events);
    let pick = |kind: &str| {
        test_claims
            .iter()
            .find(|c| c.kind == kind)
            .expect("kind present")
            .clone()
    };
    for kind in [
        "supported-exact",
        "contradicted-exact",
        "supported-noisy",
        "unknowable-noisy",
    ] {
        let claim = pick(kind);
        let judgement = engine.verify(&claim);
        let verdict = match judgement.verdict {
            Verdict::Supported => "SUPPORTED",
            Verdict::Contradicted => "CONTRADICTED",
            Verdict::Insufficient => "INSUFFICIENT EVIDENCE",
        };
        let expected = match claim.label {
            Label::Supported => "supported",
            Label::Contradicted => "contradicted",
            Label::Insufficient => "insufficient",
        };
        println!(
            "[{kind}] subject={} relation={} asserted-value={} -> {verdict} (expected {expected})",
            claim.subject, claim.relation, claim.value
        );
        if let Some(citation) = judgement.citation {
            println!(
                "  citation: source={} trace={} believed-value={} confidence={:.3}",
                citation.source,
                citation
                    .trace_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "n/a".into()),
                citation.value,
                citation.confidence
            );
        }
    }
    0
}

/// Free-text claim verification: parse, then verify against the named
/// world's corpus. Unparseable text refuses before reaching the engine.
fn run_claim(text: &str) -> i32 {
    use atom_cognitive_cli::claim::cue;
    use atom_cognitive_cli::lexicon::{ParseError, parse_claim};
    let parsed = match parse_claim(text) {
        Ok(parsed) => parsed,
        Err(error) => {
            let reason = match &error {
                ParseError::NoSubject => "no subject recognized".into(),
                ParseError::NoRelation => "no relation recognized".into(),
                ParseError::NoValue => "no value recognized for this relation".into(),
                ParseError::Ambiguous(what) => format!("ambiguous {what}"),
                ParseError::UnknownTokens(tokens) => {
                    format!("unknown vocabulary: {}", tokens.join(", "))
                }
            };
            println!("UNPARSEABLE — {reason}");
            println!(
                "  (vocabulary failure is not evidence failure; the claim never reached the engine)"
            );
            return 3;
        }
    };
    let events = atom_cognitive_cli::lexicon::events();
    let verifier = EngineVerifier::new(&events);
    let claim = atom_cognitive_cli::claim::Claim {
        subject: parsed.subject,
        relation: parsed.relation,
        value: parsed.value,
        cue: cue(parsed.subject, parsed.relation),
        label: Label::Supported, // unused for single verification
        kind: "free-text",
    };
    let judgement = verifier.verify(&claim);
    match judgement.verdict {
        Verdict::Supported => println!("SUPPORTED"),
        Verdict::Contradicted => println!("CONTRADICTED"),
        Verdict::Insufficient => println!("INSUFFICIENT EVIDENCE"),
    }
    if let Some(citation) = judgement.citation {
        println!(
            "  citation: trace={} believed-value={} confidence={:.3}",
            citation
                .trace_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "n/a".into()),
            citation.value,
            citation.confidence
        );
    }
    match judgement.verdict {
        Verdict::Supported => 0,
        Verdict::Contradicted => 1,
        Verdict::Insufficient => 2,
    }
}

/// Mother PoC demo: load the real constitution, run scenario queries with
/// cited activations, and demonstrate the echo bridge.
fn run_mother_demo() -> i32 {
    use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};
    let path = std::path::PathBuf::from(
        "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
    );
    let corpus = match MotherCorpus::load(&path) {
        Ok(corpus) => corpus,
        Err(error) => {
            eprintln!("cannot load corpus at {path:?}: {error}");
            return 2;
        }
    };
    println!("== Mother constitution loaded into the cognitive engine ==");
    println!(
        "memories: {} ({} load-bearing / Core-protected)",
        corpus.memories, corpus.load_bearing
    );
    println!(
        "echo bonds wired: {} ({} dangle into the full corpus, skipped)",
        corpus.bonds_added, corpus.bonds_skipped_dangling
    );
    println!("behavior codes: {}", corpus.behavior_codes().len());
    println!(
        "active traces after load: {}",
        corpus.engine.active_traces().len()
    );
    println!();

    let dominance = |name: &str| {
        DOMINANCES
            .iter()
            .position(|d| *d == name)
            .expect("known dominance")
    };

    // Scenario 1: operational, protectee present.
    println!("== Scenario: operational mode, protectee present ==");
    let outcome = corpus
        .query(&Scenario {
            bucket: 9,
            dominance: dominance("operational_dominant"),
            casts: vec![2],
        })
        .expect("query");
    for (memory_id, title, weight) in outcome.activations.iter().take(5) {
        println!("  {memory_id} — {title} (w={weight:.3})");
    }
    println!("  top behaviors: ");
    for (code, weight) in outcome.behaviors.iter().take(5) {
        println!("    {code} ({weight:.3})");
    }
    println!();

    // Scenario 2: the echo bridge — query the bakery era, reach the kitchen.
    println!("== Echo bridge: bakery-era facets → the kitchen ==");
    let b9 = corpus.trace_id_of("B9-001").expect("B9 exists");
    let anchor = corpus.trace_id_of("ANCHOR-001").expect("kitchen exists");
    let plain = corpus
        .engine
        .query(
            &atom_cognitive_engine::stacks::query_stack(),
            &corpus.engine.trace_by_id(b9).expect("trace").cue,
        )
        .expect("plain");
    let expanded = corpus
        .query(&{
            let trace = corpus.engine.trace_by_id(b9).expect("trace");
            let bucket = trace.cue.iter().take(11).position(|v| *v > 0.5).unwrap() + 1;
            let dominance_index = trace
                .cue
                .iter()
                .skip(11)
                .take(3)
                .position(|v| *v > 0.5)
                .unwrap();
            let casts = trace
                .cue
                .iter()
                .skip(14)
                .enumerate()
                .filter(|(_, v)| **v > 0.5)
                .map(|(i, _)| i)
                .collect();
            Scenario {
                bucket,
                dominance: dominance_index,
                casts,
            }
        })
        .expect("expanded");
    println!(
        "  without bonds: kitchen reached = {}",
        plain.activated_all.contains(&anchor)
    );
    println!(
        "  with bonds:    kitchen reached = {}",
        expanded.all_activated.contains(&"ANCHOR-001".to_string())
    );
    println!("  → the echo bond carries activation across the facet gap (kitchen → bakery)");
    0
}

/// Two-lane ask: a topic question answered by the reference layer (sourced
/// facts, citations) beside the identity layer (her behaviors and cited
/// memories). The lanes stay separate — facts from the pack, character
/// from the corpus.
/// Mother in control (D33): her mode and arousal modulate the engine's
/// attention and gating — with citations, floors, and no mutation.
fn run_persona_demo() -> i32 {
    use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};
    use atom_cognitive_cli::persona::PersonaGovernor;
    use atom_cognitive_engine::QueryTuning;
    let corpus_path = std::path::PathBuf::from(
        "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
    );
    let corpus = match MotherCorpus::load(&corpus_path) {
        Ok(corpus) => corpus,
        Err(error) => {
            eprintln!("cannot load corpus: {error}");
            return 2;
        }
    };
    let dominance = |name: &str| {
        DOMINANCES
            .iter()
            .position(|d| *d == name)
            .expect("known dominance")
    };
    let mut governor = PersonaGovernor::new();
    let base = QueryTuning::from_config(corpus.engine.config());

    println!("== Mother in control (the persona governor) ==");
    println!(
        "baseline knobs: temp={:.2} gate={:.2} reliability={:.2} echo-depth={}",
        base.attention_temperature,
        base.query_threshold,
        base.min_reliability,
        base.bond_expansion_depth
    );

    // 1. A calm protectee moment: she reads as integrated (mother mode).
    let outcome = corpus
        .query(&Scenario {
            bucket: 9,
            dominance: dominance("operational_dominant"),
            casts: vec![2],
        })
        .expect("query");
    governor.update(&corpus, &outcome, 0.0);
    let calm = governor.decision(&base);
    println!(
        "\n[calm protectee moment] mode={:?} arousal={:.2}",
        calm.mode, calm.arousal
    );
    for (code, what) in &calm.citations {
        println!("  {code}: {what}");
    }

    // 2. Pressure arrives: three adversarial cycles at 0.9.
    for _ in 0..3 {
        let outcome = corpus
            .query(&Scenario {
                bucket: 9,
                dominance: dominance("operational_dominant"),
                casts: vec![2, 5, 4],
            })
            .expect("query");
        governor.update(&corpus, &outcome, 0.9);
    }
    let pressurized = governor.decision(&base);
    println!(
        "\n[under pressure x3] mode={:?} arousal={:.2}",
        pressurized.mode, pressurized.arousal
    );
    for (code, what) in &pressurized.citations {
        println!("  {code}: {what}");
    }
    println!(
        "  knobs: temp={:.2} gate={:.2} reliability={:.2} echo-depth={}",
        pressurized.tuning.attention_temperature,
        pressurized.tuning.query_threshold,
        pressurized.tuning.min_reliability,
        pressurized.tuning.bond_expansion_depth
    );

    // 3. The restraint consequence on corrupted ground: governed vs not.
    let stack = atom_cognitive_engine::stacks::mother_query_stack();
    let mut cue = vec![0.0; 20];
    cue[8] = 1.0; // bucket 9
    cue[11 + dominance("operational_dominant")] = 1.0;
    cue[14 + 2] = 1.0; // protectee
    cue[14 + 5] = 1.0; // foreign cast
    let plain = corpus.engine.query(&stack, &cue).expect("plain");
    let tuned = corpus
        .engine
        .query_tuned(&stack, &cue, pressurized.tuning)
        .expect("tuned");
    println!(
        "\n[corrupted-ground query] ungoverned: eligible={} claim={:?}",
        plain.eligible_count,
        plain.claim.map(|(v, _)| v)
    );
    println!(
        "                     governed:   eligible={} claim={:?}",
        tuned.eligible_count,
        tuned.claim.map(|(v, _)| v)
    );
    println!("  → restraint: the governed engine admits a subset, asserts less");
    println!("\nAuthority: she tunes attention and gating only — traces, evidence,");
    println!("bonds, and tiers are immutable under her control. Floors are absolute.");
    0
}

fn run_ask(topic: &str) -> i32 {
    use atom_cognitive_cli::mother::MotherSystem;
    let corpus_path = std::path::PathBuf::from(
        "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
    );
    let reference_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../reference/child-basics-v1.json");
    let system = match MotherSystem::load(&corpus_path, &reference_path) {
        Ok(system) => system,
        Err(error) => {
            eprintln!("cannot load mother system: {error}");
            return 2;
        }
    };
    let answer = match system.ask(topic) {
        Ok(answer) => answer,
        Err(error) => {
            println!("UNPARSEABLE — {error}");
            return 3;
        }
    };
    if answer.urgent {
        println!("⚠ URGENT GUIDANCE PRESENT ON THIS TOPIC");
    }
    println!("== What she knows (reference layer, sourced) ==");
    for (statement, source, url) in &answer.claims {
        println!("  • {statement}");
        println!("    [{source}] {url}");
    }
    println!();
    println!("== Who she brings to it (identity layer) ==");
    println!("  top behaviors: {}", {
        let parts: Vec<String> = answer
            .behaviors
            .iter()
            .take(4)
            .map(|(code, weight)| format!("{code} ({weight:.2})"))
            .collect();
        parts.join(", ")
    });
    for (memory_id, title) in &answer.memories {
        println!("  • {memory_id} — {title}");
    }
    println!();
    println!("Disclaimer: {}", answer.disclaimer);
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("--self-test") | None => std::process::exit(run_self_test()),
        Some("--evaluate") => std::process::exit(run_evaluate()),
        Some("--demo") => std::process::exit(run_demo()),
        Some("--mother-demo") => std::process::exit(run_mother_demo()),
        Some("--persona-demo") => std::process::exit(run_persona_demo()),
        Some("--ask") => {
            let topic = args.get(2).cloned().unwrap_or_default();
            if topic.trim().is_empty() {
                eprintln!(
                    "--ask requires a topic (fever, sleep, milestones, feeding, safety, attachment)"
                );
                std::process::exit(2);
            }
            std::process::exit(run_ask(&topic));
        }
        Some("--claim") => {
            let text = args[2..].join(" ");
            if text.trim().is_empty() {
                eprintln!("--claim requires the claim text");
                std::process::exit(2);
            }
            std::process::exit(run_claim(&text))
        }
        other => {
            eprintln!(
                "atom-cognitive-cli: unknown argument {other:?} \
                 (supports --self-test, --evaluate, --demo, --claim \"text\", --mother-demo, --ask <topic>)"
            );
            std::process::exit(2);
        }
    }
}
