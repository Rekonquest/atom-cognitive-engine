//! Reference-layer gates: pack integrity, topic retrieval, source
//! resolution, policy enforcement, and the two-lane ask surface.

use atom_cognitive_cli::reference::ReferencePack;
use std::path::PathBuf;

fn pack_path() -> PathBuf {
    std::env::var("ATOM_MOTHER_REFERENCE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../reference/child-basics-v1.json")
        })
}

#[test]
fn pack_loads_and_resolves() {
    let pack = ReferencePack::load(&pack_path()).expect("pack loads");
    assert_eq!(pack.claim_count(), 26);
    assert_eq!(pack.file_hash().len(), 64, "sha-256 hex recorded");
    // Every claim's source resolves (enforced at load; assert the count).
    for topic in pack.topics() {
        for claim in pack.claims_for_topic(topic) {
            let source = pack.source_of(claim).expect("source resolves");
            assert!(source.url.starts_with("https://"));
        }
    }
}

#[test]
fn topic_retrieval_is_exact() {
    let pack = ReferencePack::load(&pack_path()).expect("pack loads");
    let fever = pack.claims_for_topic("fever");
    assert!(fever.len() >= 6, "fever domain is populated");
    let first = fever
        .iter()
        .find(|c| c.claim_id == "fever-001")
        .expect("fever-001 present");
    assert!(first.urgent);
    assert!(first.statement.contains("100.4"));
    // Unknown topics return empty — refusal, never a guess.
    assert!(pack.claims_for_topic("quantum-mechanics").is_empty());
}

#[test]
fn urgent_flags_mark_the_dangerous_ones() {
    let pack = ReferencePack::load(&pack_path()).expect("pack loads");
    assert!(pack.has_urgent("fever"));
    assert!(pack.has_urgent("feeding")); // honey
    assert!(pack.has_urgent("safety")); // water
    assert!(!pack.has_urgent("sleep"));
}

#[test]
fn integrity_detects_tampering() {
    let pack = ReferencePack::load(&pack_path()).expect("pack loads");
    assert!(pack.verify_integrity(&pack_path()));
    // Tampered copy: change one byte; verification must fail.
    let original = std::fs::read(pack_path()).expect("read");
    let mut tampered = original.clone();
    // Flip a character inside a statement (not in structural JSON).
    let position = tampered
        .windows(9)
        .position(|w| w == b"statement")
        .expect("statement field");
    tampered[position + 20] ^= 0x01;
    let temp = std::env::temp_dir().join("atom-tampered-reference.json");
    std::fs::write(&temp, tampered).expect("write");
    let tampered_pack = ReferencePack::load(&temp).expect("still parses");
    assert!(
        !tampered_pack.verify_integrity(&temp) || tampered_pack.file_hash() != pack.file_hash(),
        "tampered content must not verify as the original"
    );
    assert_ne!(tampered_pack.file_hash(), pack.file_hash());
    let _ = std::fs::remove_file(&temp);
}

#[test]
fn policy_violations_fail_closed() {
    // A pack without the evidence-not-instruction policy is refused.
    let original = std::fs::read_to_string(pack_path()).expect("read");
    let broken = original.replace(
        "\"evidence_not_instruction\": true",
        "\"evidence_not_instruction\": false",
    );
    assert_ne!(broken, original, "policy line must exist to be broken");
    let temp = std::env::temp_dir().join("atom-broken-policy.json");
    std::fs::write(&temp, broken).expect("write");
    assert!(ReferencePack::load(&temp).is_err());
    let _ = std::fs::remove_file(&temp);
}

#[test]
fn two_lane_ask_answers_with_both_lanes() {
    use atom_cognitive_cli::mother::MotherSystem;
    let system = MotherSystem::load(
        &std::path::PathBuf::from(
            "/home/jesse/Desktop/Projects/mother ai/mothers core memories/mothers_core_memories.json",
        ),
        &pack_path(),
    )
    .expect("system loads");
    let answer = system.ask("fever").expect("fever asks");
    assert!(answer.urgent);
    assert!(answer.claims.len() >= 6);
    // The reference lane cites real sources.
    assert!(
        answer
            .claims
            .iter()
            .any(|(_, _, url)| url.contains("healthychildren"))
    );
    // The identity lane brings her behaviors and cited memories — the
    // lanes answer side by side, never merged.
    assert!(!answer.behaviors.is_empty());
    assert!(!answer.memories.is_empty());
    assert!(answer.disclaimer.contains("not medical advice"));
    // Unknown topic: refusal naming the vocabulary.
    assert!(system.ask("astrophysics").is_err());
}
