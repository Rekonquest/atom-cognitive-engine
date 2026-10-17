//! Gates for the hybrid: each one names the law it holds.

use atom_cognitive_rag::{AbstainReason, CognitiveRag, Verdict};
use std::path::PathBuf;

fn store(name: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("cognitive-rag-{name}-{}.atoms", std::process::id()));
    let _ = std::fs::remove_file(&path);
    path
}

fn seeded(name: &str) -> (CognitiveRag, PathBuf) {
    let path = store(name);
    let mut rag = CognitiveRag::open(&path).unwrap();
    rag.remember(
        "observer-leases.md",
        "A writer crash releases its lease. Lease recovery permits a replacement writer \
         immediately, and observers never repair provisional bytes.",
    )
    .unwrap();
    rag.remember(
        "sourdough.md",
        "A sourdough starter needs flour and water every day. Bake the bread in a hot \
         covered pot so the crust turns thick and crackles.",
    )
    .unwrap();
    rag.remember(
        "lease-myth.md",
        "Folk advice says a lease on an apartment ends after a crash in the housing market; \
         writers of such advice rarely cite recovery data.",
    )
    .unwrap();
    (rag, path)
}

#[test]
fn coherent_evidence_is_grounded_and_cited() {
    let (mut rag, path) = seeded("grounded");
    let answer = rag
        .ask("how does writer lease recovery work after a crash")
        .unwrap();
    assert!(
        matches!(answer.verdict, Verdict::Grounded { .. }),
        "{answer:?}"
    );
    let top = &answer.citations[0];
    assert_eq!(top.sources, vec!["observer-leases.md".to_string()]);
    assert!(top.engine_rank.is_some());
    // Every grounded citation passed the engine's gate.
    assert!(answer.citations.iter().all(|c| c.engine_rank.is_some()));
    let _ = std::fs::remove_file(path);
}

#[test]
fn unknown_ground_abstains_without_evidence() {
    let (mut rag, path) = seeded("unknown");
    let answer = rag.ask("quantum chromodynamics gluon").unwrap();
    assert_eq!(answer.verdict, Verdict::Abstain(AbstainReason::NoEvidence));
    assert!(answer.citations.is_empty());
    let _ = std::fs::remove_file(path);
}

#[test]
fn word_overlap_without_coherence_is_not_an_answer() {
    // AtomDB's cues converge (bread, lease, crash all exist), but no local
    // window in any passage holds the question together.
    let (mut rag, path) = seeded("overlap");
    let answer = rag.ask("bread lease crash thick").unwrap();
    assert!(answer.shortlist > 0, "AtomDB should still shortlist");
    assert_eq!(
        answer.verdict,
        Verdict::Abstain(AbstainReason::EngineUncertain),
        "{answer:?}"
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn misleading_feedback_is_durable_and_revises_judgment() {
    let (mut rag, path) = seeded("feedback");
    let query = "how does writer lease recovery work after a crash";
    let before = rag.ask(query).unwrap();
    let target = before.citations[0].passage;
    for _ in 0..4 {
        rag.feedback(target, false).unwrap();
    }
    drop(rag);

    // Reopen: feedback is a fact in AtomDB, not process state.
    let mut rag = CognitiveRag::open(&path).unwrap();
    assert_eq!(rag.feedback_counts(target).unwrap(), (0, 4));
    let after = rag.ask(query).unwrap();
    // The engine replays the four marks as contradictions: the passage that
    // grounded the answer now testifies against itself, so it abstains.
    assert_eq!(
        after.verdict,
        Verdict::Abstain(AbstainReason::Misleading),
        "{after:?}"
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn asks_are_read_only_and_deterministic() {
    let (mut rag, path) = seeded("determinism");
    let stats = rag.db().stats().unwrap();
    let a = rag.ask("writer lease recovery crash").unwrap();
    let b = rag.ask("writer lease recovery crash").unwrap();
    assert_eq!(a.response_hash, b.response_hash);
    assert_eq!(a.to_json(), b.to_json());
    assert_eq!(rag.db().stats().unwrap(), stats, "asking must not write");
    let _ = std::fs::remove_file(path);
}
