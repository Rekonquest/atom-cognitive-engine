//! Diagnostic: inspect the Mother corpus load and the failing gates.

use atom_cognitive_cli::mother::{DOMINANCES, MotherCorpus, Scenario};

fn main() {
    let path = std::path::PathBuf::from(
        "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
    );
    let corpus = MotherCorpus::load(&path).expect("loads");
    println!(
        "memories={} load_bearing={} bonds={} dangling={} behaviors={} active={}",
        corpus.memories,
        corpus.load_bearing,
        corpus.bonds_added,
        corpus.bonds_skipped_dangling,
        corpus.behavior_codes().len(),
        corpus.engine.active_traces().len()
    );
    let core_min = corpus
        .engine
        .active_traces()
        .iter()
        .filter(|t| t.tier == atom_cognitive_engine::TraceTier::Core)
        .map(|t| t.support)
        .fold(f64::INFINITY, f64::min);
    let standard_min = corpus
        .engine
        .active_traces()
        .iter()
        .filter(|t| t.tier == atom_cognitive_engine::TraceTier::Standard)
        .map(|t| t.support)
        .fold(f64::INFINITY, f64::min);
    println!("core_min_support={core_min:.4} standard_min_support={standard_min:.4}");

    // B9-001 facets: plain query activations.
    let b9_trace = corpus
        .engine
        .trace_by_id(corpus.trace_id_of("B9-001").expect("exists"))
        .expect("trace");
    let plain = corpus
        .engine
        .query(&atom_cognitive_engine::stacks::query_stack(), &b9_trace.cue)
        .expect("plain");
    println!(
        "plain B9 query: claim={:?} eligible={} activations={:?}",
        plain.claim.map(|(v, w)| (v, (w * 1000.0).round() / 1000.0)),
        plain.eligible_count,
        plain
            .activations
            .iter()
            .map(|(id, w)| (id, (w * 1000.0).round() / 1000.0))
            .collect::<Vec<_>>()
    );
    // How many traces share B9's exact cue? (same bucket+dominance+cast)
    let same_cue = corpus
        .engine
        .active_traces()
        .iter()
        .filter(|t| t.cue == b9_trace.cue)
        .count();
    println!("traces sharing B9's exact cue: {same_cue}");

    // Bucket-9 operational scenario with protectee.
    let scenario = Scenario {
        bucket: 9,
        dominance: DOMINANCES
            .iter()
            .position(|d| *d == "operational_dominant")
            .expect("dominance"),
        casts: vec![2],
    };
    let outcome = corpus.query(&scenario).expect("query");
    println!("scenario activations:");
    for (memory_id, title, weight) in outcome.activations.iter().take(8) {
        println!("  {memory_id} ({title}) w={weight:.3}");
    }
    println!("scenario behaviors: {:?}", outcome.behaviors);
    println!("abstained={}", outcome.abstained);
}
