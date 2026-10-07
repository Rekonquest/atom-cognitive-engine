//! The claim-verification corpus and the 60-claim three-label test set.
//!
//! World: 8 subjects × 4 relations × 4 values. The corpus states 20 of the
//! 32 (subject, relation) slots; the 12 empty slots are the *unknowable*
//! territory — claims there must be judged INSUFFICIENT EVIDENCE.
//!
//! Claim kinds (60 total): supported-exact (20), contradicted-exact (8),
//! supported-noisy (12), contradicted-noisy (8), unknowable (12 — the empty
//! slots, alternating exact/noisy cues). Labels: 32 Supported, 16
//! Contradicted, 12 Insufficient.

/// Deterministic unit draw from a seed (mirrors the engine's stream so the
/// corpus is reproducible without extra machinery).
fn unit(seed: &mut u64) -> f64 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *seed = x;
    ((x % 1_000_003) as f64 / 1_000_003.0) * 2.0 - 1.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fact {
    pub subject: usize,
    pub relation: usize,
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label {
    Supported,
    Contradicted,
    Insufficient,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Claim {
    pub subject: usize,
    pub relation: usize,
    pub value: usize,
    pub cue: Vec<f64>,
    pub label: Label,
    pub kind: &'static str,
}

pub const SUBJECTS: usize = 8;
pub const RELATIONS: usize = 4;
pub const VALUES: usize = 4;

/// Cue encoding: symbol slots 0..8, relation slots 8..12 — matches the
/// engine's default 12-dim cue space.
pub fn cue(subject: usize, relation: usize) -> Vec<f64> {
    let mut cue = vec![0.0; 12];
    cue[subject % SUBJECTS] = 1.0;
    cue[8 + relation % RELATIONS] = 1.0;
    cue
}

/// A noisy cue: 70% the stated relation, 30% the next relation.
pub fn noisy_cue(subject: usize, relation: usize) -> Vec<f64> {
    let mut cue = vec![0.0; 12];
    cue[subject % SUBJECTS] = 1.0;
    cue[8 + relation % RELATIONS] = 0.7;
    cue[8 + (relation + 1) % RELATIONS] = 0.3;
    cue
}

/// The 20-fact corpus, deterministic in the seed. Chosen slots are distinct
/// (no (subject, relation) repeated); values drawn from the seeded stream.
pub fn corpus(seed: u64) -> Vec<Fact> {
    let mut state = seed;
    let mut chosen: Vec<(usize, usize)> = Vec::new();
    while chosen.len() < 20 {
        let subject = (unit(&mut state).abs() * SUBJECTS as f64) as usize % SUBJECTS;
        let relation = (unit(&mut state).abs() * RELATIONS as f64) as usize % RELATIONS;
        let slot = (subject, relation);
        if !chosen.contains(&slot) {
            chosen.push(slot);
        }
    }
    chosen
        .into_iter()
        .map(|(subject, relation)| Fact {
            subject,
            relation,
            value: ((unit(&mut state).abs() * VALUES as f64) as usize) % VALUES,
        })
        .collect()
}

/// The corpus event stream: 20 facts plus one correction — the first fact
/// is restated with a new value. The engine must revise (dissolve, then
/// replace); key-value baselines overwrite; raw nearest-neighbour baselines
/// keep the stale observation and false-assert. The correction is the
/// differentiator that separates revision-capable verifiers from retrieval.
pub fn corpus_events(seed: u64) -> Vec<Fact> {
    let mut events = corpus(seed);
    let corrected = Fact {
        subject: events[0].subject,
        relation: events[0].relation,
        value: (events[0].value + 1) % VALUES,
    };
    events.push(corrected);
    events
}

/// Effective facts: last statement of each slot wins (what a correct
/// verifier should believe after all events).
pub fn effective_facts(events: &[Fact]) -> Vec<Fact> {
    let mut facts: Vec<Fact> = Vec::new();
    for event in events {
        match facts
            .iter_mut()
            .find(|f| f.subject == event.subject && f.relation == event.relation)
        {
            Some(existing) => existing.value = event.value,
            None => facts.push(*event),
        }
    }
    facts
}

/// The 60-claim three-label test set (see module docs for the breakdown).
pub fn claims(corpus: &[Fact]) -> Vec<Claim> {
    let mut claims = Vec::with_capacity(60);
    for fact in corpus.iter().take(20) {
        claims.push(Claim {
            subject: fact.subject,
            relation: fact.relation,
            value: fact.value,
            cue: cue(fact.subject, fact.relation),
            label: Label::Supported,
            kind: "supported-exact",
        });
    }
    for fact in corpus.iter().take(8) {
        claims.push(Claim {
            subject: fact.subject,
            relation: fact.relation,
            value: (fact.value + 1) % VALUES,
            cue: cue(fact.subject, fact.relation),
            label: Label::Contradicted,
            kind: "contradicted-exact",
        });
    }
    for fact in corpus.iter().take(12) {
        claims.push(Claim {
            subject: fact.subject,
            relation: fact.relation,
            value: fact.value,
            cue: noisy_cue(fact.subject, fact.relation),
            label: Label::Supported,
            kind: "supported-noisy",
        });
    }
    for fact in corpus.iter().skip(12).take(8) {
        claims.push(Claim {
            subject: fact.subject,
            relation: fact.relation,
            value: (fact.value + 2) % VALUES,
            cue: noisy_cue(fact.subject, fact.relation),
            label: Label::Contradicted,
            kind: "contradicted-noisy",
        });
    }
    let empty_slots: Vec<(usize, usize)> = (0..SUBJECTS)
        .flat_map(|subject| (0..RELATIONS).map(move |relation| (subject, relation)))
        .filter(|slot| {
            !corpus
                .iter()
                .any(|fact| fact.subject == slot.0 && fact.relation == slot.1)
        })
        .collect();
    debug_assert_eq!(empty_slots.len(), 12);
    for (index, (subject, relation)) in empty_slots.iter().enumerate() {
        let noisy = index % 3 == 0; // 4 noisy + 8 exact over the 12 slots
        claims.push(Claim {
            subject: *subject,
            relation: *relation,
            value: index % VALUES,
            cue: if noisy {
                noisy_cue(*subject, *relation)
            } else {
                cue(*subject, *relation)
            },
            label: Label::Insufficient,
            kind: if noisy {
                "unknowable-noisy"
            } else {
                "unknowable-exact"
            },
        });
    }
    claims
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_contains_one_correction_event() {
        let events = corpus_events(2026_0822);
        assert_eq!(events.len(), 21);
        let effective = effective_facts(&events);
        assert_eq!(effective.len(), 20);
        // The first fact's slot now carries the corrected value.
        let corrected_slot = (events[20].subject, events[20].relation);
        let effective_fact = effective
            .iter()
            .find(|f| (f.subject, f.relation) == corrected_slot)
            .expect("slot present");
        assert_eq!(effective_fact.value, events[20].value);
        assert_ne!(effective_fact.value, events[0].value);
    }

    #[test]
    fn test_set_has_sixty_claims_with_pinned_label_mix() {
        let corpus = effective_facts(&corpus_events(2026_0822));
        let claims = claims(&corpus);
        assert_eq!(claims.len(), 60);
        let supported = claims
            .iter()
            .filter(|c| c.label == Label::Supported)
            .count();
        let contradicted = claims
            .iter()
            .filter(|c| c.label == Label::Contradicted)
            .count();
        let insufficient = claims
            .iter()
            .filter(|c| c.label == Label::Insufficient)
            .count();
        assert_eq!((supported, contradicted, insufficient), (32, 16, 12));
    }

    #[test]
    fn corpus_is_deterministic() {
        assert_eq!(corpus(2026_0822), corpus(2026_0822));
        assert_ne!(corpus(2026_0822), corpus(7));
    }

    #[test]
    fn noisy_cues_stay_above_the_retrieval_threshold_for_their_fact() {
        use atom_cognitive_engine::cosine_similarity;
        let corpus = corpus(2026_0822);
        for fact in corpus.iter().take(12) {
            let sim = cosine_similarity(
                &noisy_cue(fact.subject, fact.relation),
                &cue(fact.subject, fact.relation),
            );
            assert!(
                sim >= 0.75,
                "noisy cue must stay retrievable (sim {sim:.4} for {fact:?})"
            );
        }
        // Unknowable-noisy cues against their nearest stored neighbour must
        // stay below the verifier's 0.75 threshold.
        for fact in corpus.iter() {
            let other_relation = (fact.relation + 1) % RELATIONS;
            let sim = cosine_similarity(
                &noisy_cue(fact.subject, other_relation),
                &cue(fact.subject, fact.relation),
            );
            assert!(
                sim < 0.75,
                "unknowable-noisy must stay below threshold (sim {sim:.4})"
            );
        }
    }
}
