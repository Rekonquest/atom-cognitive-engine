//! W14's with/without law applied to the validated surface: the full
//! 60-claim benchmark re-run with robust aggregation ON must hold 60/60
//! with zero false assertions — the defense may not cost the honest
//! answers.

use atom_cognitive_cli::claim::{claims, corpus_events, effective_facts};
use atom_cognitive_cli::verify::{EngineVerifier, evaluate};
use atom_cognitive_engine::EngineConfig;

#[test]
fn claim_benchmark_holds_under_robust_aggregation() {
    let events = corpus_events(2026_0822);
    let test_claims = claims(&effective_facts(&events));
    let config = EngineConfig {
        capacity: 32,
        query_threshold: 0.75,
        initial_ttl: 32,
        ttl_per_hit: 8,
        robust_weight_cap: 0.5, // the defense, on
        robust_trim: 1,
        ..EngineConfig::default()
    };
    let verifier = EngineVerifier::new_tuned(&events, config);
    let score = evaluate(&verifier, &test_claims);
    assert_eq!(
        score.correct, 60,
        "robust aggregation must not cost honest answers (misses: {}, false: {})",
        score.misses, score.false_assertions
    );
    assert_eq!(score.false_assertions, 0);
    assert_eq!(score.correct_abstentions, 12);
}
