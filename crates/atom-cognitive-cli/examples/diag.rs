//! Diagnostic: print every claim where the engine's verdict disagrees with
//! its label, with similarities against all active traces.

use atom_cognitive_cli::claim::{Label, claims, corpus_events, cue, effective_facts};
use atom_cognitive_cli::verify::{ClaimVerifier, EngineVerifier, Verdict, score};
use atom_cognitive_engine::cosine_similarity;

fn main() {
    let events = corpus_events(2026_0822);
    let test_claims = claims(&effective_facts(&events));
    let verifier = EngineVerifier::new(&events);
    for claim in &test_claims {
        let judgement = verifier.verify(claim);
        let scored = score(judgement.verdict, claim.label);
        if scored != atom_cognitive_cli::verify::Score::Correct {
            println!(
                "MISMATCH kind={} label={:?} verdict={:?} subject={} relation={} value={}",
                claim.kind,
                claim.label,
                judgement.verdict,
                claim.subject,
                claim.relation,
                claim.value
            );
            println!("  cue: {:?}", claim.cue);
            for trace in verifier.active_traces() {
                let sim = cosine_similarity(&claim.cue, &trace.cue);
                if sim > 0.3 {
                    println!(
                        "  trace {} sim={:.4} value={:?} reliability={:.3} hits={} support={:.3}",
                        trace.id,
                        sim,
                        trace.value(),
                        trace.reliability(),
                        trace.hits,
                        trace.support
                    );
                }
            }
        }
    }
    let _ = Verdict::Supported; // silence unused import if any
    let _ = Label::Supported;
    let _ = cue(0, 0);
}
