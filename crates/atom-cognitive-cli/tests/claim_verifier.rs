//! The Phase 2 pinned-criterion benchmark: the engine versus three
//! baselines on the 60-claim three-label test set.

use atom_cognitive_cli::claim::{Label, claims, corpus_events, effective_facts};
use atom_cognitive_cli::verify::{
    ClaimVerifier, EngineVerifier, ExactAddressVerifier, NearestVerifier, Verdict, evaluate,
};

fn test_claims() -> Vec<atom_cognitive_cli::claim::Claim> {
    claims(&effective_facts(&corpus_events(2026_0822)))
}

#[test]
fn engine_meets_the_pinned_criterion() {
    // D10, pinned in advance: beat every baseline on false assertions while
    // matching or beating every baseline on correct answers.
    let events = corpus_events(2026_0822);
    let test_claims = test_claims();
    let engine = EngineVerifier::new(&events);
    let systems = [
        evaluate(&engine, &test_claims),
        evaluate(&ExactAddressVerifier::new(&events), &test_claims),
        evaluate(&NearestVerifier::new(&events, false), &test_claims),
        evaluate(&NearestVerifier::new(&events, true), &test_claims),
    ];
    let engine_score = &systems[0];
    for baseline in &systems[1..] {
        assert!(
            engine_score.false_assertions <= baseline.false_assertions,
            "engine ({}) must not false-assert more than {} ({})",
            engine_score.false_assertions,
            baseline.name,
            baseline.false_assertions
        );
        assert!(
            engine_score.correct >= baseline.correct,
            "engine ({}) must match or beat {} on correct answers ({})",
            engine_score.correct,
            baseline.name,
            baseline.correct
        );
    }
    // The strict form: zero false assertions at full coverage.
    assert_eq!(engine_score.false_assertions, 0);
    assert_eq!(engine_score.correct, 60);
}

#[test]
fn baselines_fail_where_expected() {
    let events = corpus_events(2026_0822);
    let test_claims = test_claims();
    // Exact-address cannot retrieve noisy cues: coverage misses.
    let exact = evaluate(&ExactAddressVerifier::new(&events), &test_claims);
    assert_eq!(exact.misses, 20, "all 20 noisy claims are missed");
    assert_eq!(exact.false_assertions, 0);
    // Nearest-always asserts on unknowable ground and on the stale
    // correction observation.
    let always = evaluate(&NearestVerifier::new(&events, false), &test_claims);
    assert!(
        always.false_assertions >= 12,
        "unknowable ground must produce false assertions, got {}",
        always.false_assertions
    );
    // Nearest-thresholded refuses unknowables but keeps the stale
    // observation: the revision failure the engine's REVISE root fixes.
    let thresholded = evaluate(&NearestVerifier::new(&events, true), &test_claims);
    assert!(
        thresholded.false_assertions >= 1,
        "the stale correction observation must false-assert the raw retriever"
    );
}

#[test]
fn engine_verdicts_are_cited_and_abstentions_are_clean() {
    let events = corpus_events(2026_0822);
    let test_claims = test_claims();
    let engine = EngineVerifier::new(&events);
    for claim in &test_claims {
        let judgement = engine.verify(claim);
        match judgement.verdict {
            Verdict::Insufficient => {
                assert!(judgement.citation.is_none(), "abstentions never cite");
                assert_eq!(claim.label, Label::Insufficient);
            }
            Verdict::Supported | Verdict::Contradicted => {
                let citation = judgement.citation.as_ref().expect("assertions always cite");
                assert!(citation.trace_id.is_some(), "engine citations name a trace");
                assert!(citation.confidence > 0.5);
            }
        }
    }
}

#[test]
fn evaluation_is_deterministic() {
    let events = corpus_events(2026_0822);
    let test_claims = test_claims();
    let run = || {
        let engine = EngineVerifier::new(&events);
        (engine.state_hash(), evaluate(&engine, &test_claims))
    };
    assert_eq!(run(), run());
}
