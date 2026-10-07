//! End-to-end free-text verification over the authored named world.

use atom_cognitive_cli::claim::{Claim, Label, cue};
use atom_cognitive_cli::lexicon::{events, parse_claim};
use atom_cognitive_cli::verify::{ClaimVerifier, EngineVerifier, Verdict};

fn verify_text(text: &str) -> Verdict {
    let parsed = parse_claim(text).expect("claims in this test parse");
    let verifier = EngineVerifier::new(&events());
    let claim = Claim {
        subject: parsed.subject,
        relation: parsed.relation,
        value: parsed.value,
        cue: cue(parsed.subject, parsed.relation),
        label: Label::Supported,
        kind: "free-text",
    };
    verifier.verify(&claim).verdict
}

#[test]
fn supported_free_text_claims() {
    assert_eq!(
        verify_text("The Atlas range is in Africa"),
        Verdict::Supported
    );
    assert_eq!(verify_text("andes weather tropical"), Verdict::Supported);
}

#[test]
fn corrected_survey_overrides_the_early_one() {
    // The engine believes the revised survey (medium), so the corrected
    // value is supported and the early value is contradicted.
    assert_eq!(verify_text("cascade length medium"), Verdict::Supported);
    assert_eq!(verify_text("cascade length long"), Verdict::Contradicted);
}

#[test]
fn unstated_ground_abstains() {
    // The cascades' climate was never stated.
    assert_eq!(verify_text("cascade climate polar"), Verdict::Insufficient);
    // Neither was anything about the carpathians.
    assert_eq!(
        verify_text("carpathian continent europe"),
        Verdict::Insufficient
    );
}

#[test]
fn wrong_value_claims_contradict() {
    assert_eq!(verify_text("atlas continent asia"), Verdict::Contradicted);
}

#[test]
fn unparseable_text_never_reaches_the_engine() {
    assert!(atom_cognitive_cli::lexicon::parse_claim("the moon is cheese").is_err());
    assert!(atom_cognitive_cli::lexicon::parse_claim("atlas continent africa today").is_err());
}
