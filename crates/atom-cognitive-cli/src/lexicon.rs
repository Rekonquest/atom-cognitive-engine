//! Free-text claim surface: a named world, an authored corpus, and a
//! deterministic exact-vocabulary parser.
//!
//! The parser follows the atom-harness's lexical-anchor discipline: tokens
//! are matched against a fixed alias vocabulary; unknown or ambiguous text
//! REFUSES to parse rather than guessing. An unparseable claim never
//! reaches the engine — vocabulary failure is not evidence failure.

/// A named world entity with lookup aliases.
pub struct Named {
    pub id: usize,
    pub name: &'static str,
    pub aliases: &'static [&'static str],
}

pub const SUBJECTS: [Named; 8] = [
    Named {
        id: 0,
        name: "atlas",
        aliases: &["atlas"],
    },
    Named {
        id: 1,
        name: "andes",
        aliases: &["andes"],
    },
    Named {
        id: 2,
        name: "urals",
        aliases: &["urals", "ural"],
    },
    Named {
        id: 3,
        name: "rockies",
        aliases: &["rockies", "rocky"],
    },
    Named {
        id: 4,
        name: "alps",
        aliases: &["alps", "alpine"],
    },
    Named {
        id: 5,
        name: "himalayas",
        aliases: &["himalayas", "himalaya"],
    },
    Named {
        id: 6,
        name: "cascades",
        aliases: &["cascades", "cascade"],
    },
    Named {
        id: 7,
        name: "carpathians",
        aliases: &["carpathians", "carpathian"],
    },
];

pub const RELATIONS: [Named; 4] = [
    Named {
        id: 0,
        name: "continent",
        aliases: &["continent", "located-on"],
    },
    Named {
        id: 1,
        name: "climate",
        aliases: &["climate", "weather"],
    },
    Named {
        id: 2,
        name: "length-class",
        aliases: &["length-class", "length", "size"],
    },
    Named {
        id: 3,
        name: "formation",
        aliases: &["formation", "formed", "geology"],
    },
];

/// Per-relation value names; index within the row is the engine value id.
pub const VALUE_NAMES: [[&str; 4]; 4] = [
    ["africa", "south-america", "europe", "asia"],
    ["arid", "temperate", "polar", "tropical"],
    ["short", "medium", "long", "massive"],
    ["volcanic", "folded", "fault-block", "plateau"],
];

/// The initial 20 authored facts (distinct slots). The cascades' length
/// class appears here as the EARLY survey; `events()` appends the corrected
/// survey. Slots 6 and 7 are otherwise unstated — unknowable ground.
pub const INITIAL_EVENTS: [(usize, usize, usize); 20] = [
    (0, 0, 0), // atlas continent africa
    (0, 1, 0), // atlas climate arid
    (0, 2, 1), // atlas length medium
    (0, 3, 1), // atlas formation folded
    (1, 0, 1), // andes continent south-america
    (1, 1, 3), // andes climate tropical
    (1, 2, 3), // andes length massive
    (1, 3, 0), // andes formation volcanic
    (2, 0, 2), // urals continent europe
    (2, 1, 1), // urals climate temperate
    (2, 2, 1), // urals length medium
    (3, 0, 1), // rockies continent south-america
    (3, 1, 1), // rockies climate temperate
    (3, 2, 2), // rockies length long
    (3, 3, 2), // rockies formation fault-block
    (4, 0, 2), // alps continent europe
    (4, 1, 1), // alps climate temperate
    (4, 2, 1), // alps length medium
    (5, 0, 3), // himalayas continent asia
    (6, 2, 2), // cascades length long (early survey)
];

/// Full event stream including the correction (cascades length: long →
/// medium, the revised survey).
pub fn events() -> Vec<crate::claim::Fact> {
    let mut stream: Vec<crate::claim::Fact> = INITIAL_EVENTS
        .iter()
        .map(|(s, r, v)| crate::claim::Fact {
            subject: *s,
            relation: *r,
            value: *v,
        })
        .collect();
    stream.push(crate::claim::Fact {
        subject: 6,
        relation: 2,
        value: 1,
    });
    stream
}

const STOPWORDS: [&str; 12] = [
    "the",
    "a",
    "an",
    "is",
    "in",
    "of",
    "its",
    "has",
    "range",
    "mountain",
    "mountains",
    "lies",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    NoSubject,
    NoRelation,
    NoValue,
    Ambiguous(&'static str),
    UnknownTokens(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedClaim {
    pub subject: usize,
    pub relation: usize,
    pub value: usize,
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty() && !STOPWORDS.contains(token))
        .map(String::from)
        .collect()
}

/// One vocabulary hit: entity id, its alias normalized to space-separated
/// tokens, and the concrete tokens consumed from the input.
struct Hit {
    id: usize,
    alias_tokens: Vec<String>,
}

fn find_hits(tokens: &[String], entities: &[Named]) -> Vec<Hit> {
    let mut hits = Vec::new();
    let mut spans: Vec<Vec<String>> = Vec::new(); // consumed token sequences
    for token in tokens {
        spans.push(vec![token.clone()]);
    }
    for window in tokens.windows(2) {
        spans.push(vec![window[0].clone(), window[1].clone()]);
    }
    for named in entities {
        for alias in named.aliases {
            let alias_tokens: Vec<String> = alias
                .replace('-', " ")
                .split_whitespace()
                .map(String::from)
                .collect();
            for span in &spans {
                if *span == alias_tokens {
                    hits.push(Hit {
                        id: named.id,
                        alias_tokens: alias_tokens.clone(),
                    });
                }
            }
        }
    }
    hits
}

fn resolve(hits: &[Hit], what: &'static str) -> Result<usize, ParseError> {
    if hits.is_empty() {
        return Err(match what {
            "subject" => ParseError::NoSubject,
            "relation" => ParseError::NoRelation,
            _ => ParseError::NoValue,
        });
    }
    let first = hits[0].id;
    if hits.iter().all(|hit| hit.id == first) {
        Ok(first)
    } else {
        Err(ParseError::Ambiguous(what))
    }
}

/// Parse free text into a structured claim. Refuses on unknown vocabulary,
/// ambiguity, or missing parts — never guesses.
pub fn parse_claim(text: &str) -> Result<ParsedClaim, ParseError> {
    let tokens = tokenize(text);
    let subject_hits = find_hits(&tokens, &SUBJECTS);
    let relation_hits = find_hits(&tokens, &RELATIONS);
    let subject = resolve(&subject_hits, "subject")?;
    // Relation inference: when no relation word appears, a value token that
    // belongs to exactly ONE relation's value vocabulary implies that
    // relation ("atlas is in africa" → continent). The value sets are
    // disjoint, so the inference is deterministic and vocabulary-bounded —
    // the lexical-anchor pattern: vocabulary may route, never invent.
    let relation = if relation_hits.is_empty() {
        let mut candidates: Vec<usize> = Vec::new();
        for (relation, names) in VALUE_NAMES.iter().enumerate() {
            let entities: Vec<Named> = names
                .iter()
                .enumerate()
                .map(|(id, name)| Named {
                    id,
                    name,
                    aliases: std::slice::from_ref(name),
                })
                .collect();
            if !find_hits(&tokens, &entities).is_empty() {
                candidates.push(relation);
            }
        }
        if candidates.len() == 1 {
            candidates[0]
        } else {
            return Err(ParseError::NoRelation);
        }
    } else {
        resolve(&relation_hits, "relation")?
    };
    let value_entities: Vec<Named> = VALUE_NAMES[relation]
        .iter()
        .enumerate()
        .map(|(id, name)| Named {
            id,
            name,
            aliases: std::slice::from_ref(name),
        })
        .collect();
    let value_hits = find_hits(&tokens, &value_entities);
    let value = resolve(&value_hits, "value")?;
    // Every token must be consumed by some matched alias; leftovers are
    // unknown vocabulary and the parse refuses.
    let consumed: Vec<String> = subject_hits
        .iter()
        .chain(relation_hits.iter())
        .chain(value_hits.iter())
        .flat_map(|hit| hit.alias_tokens.iter().cloned())
        .collect();
    let leftovers: Vec<String> = tokens
        .iter()
        .filter(|token| !consumed.contains(token))
        .cloned()
        .collect();
    if !leftovers.is_empty() {
        return Err(ParseError::UnknownTokens(leftovers));
    }
    Ok(ParsedClaim {
        subject,
        relation,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_and_styled_claims() {
        let parsed = parse_claim("The Atlas range is in Africa").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 0,
                relation: 0,
                value: 0
            }
        );
        let parsed = parse_claim("himalaya weather polar").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 5,
                relation: 1,
                value: 2
            }
        );
        // The corrected survey value parses too.
        let parsed = parse_claim("cascade length medium").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 6,
                relation: 2,
                value: 1
            }
        );
    }

    #[test]
    fn parses_hyphenated_bigrams() {
        let parsed = parse_claim("andes continent south america").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 1,
                relation: 0,
                value: 1
            }
        );
        let parsed = parse_claim("rocky geology fault block").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 3,
                relation: 3,
                value: 2
            }
        );
    }

    #[test]
    fn refuses_missing_parts() {
        assert!(matches!(
            parse_claim("cheese everywhere"),
            Err(ParseError::NoSubject)
        ));
        assert!(matches!(
            parse_claim("atlas continent cheese"),
            Err(ParseError::NoValue)
        ));
        // No relation word and no inferable value: refusal.
        assert!(matches!(
            parse_claim("atlas cheese"),
            Err(ParseError::NoRelation)
        ));
    }

    #[test]
    fn infers_relation_from_value_vocabulary() {
        // "is in africa" carries no relation word; "africa" belongs only to
        // the continent vocabulary, so the relation is inferred.
        let parsed = parse_claim("atlas is in africa").expect("parse");
        assert_eq!(
            parsed,
            ParsedClaim {
                subject: 0,
                relation: 0,
                value: 0
            }
        );
    }

    #[test]
    fn refuses_unknown_and_ambiguous() {
        // A full parse with a leftover unknown word refuses on vocabulary.
        assert!(matches!(
            parse_claim("atlas continent africa zaptron"),
            Err(ParseError::UnknownTokens(tokens)) if tokens == vec!["zaptron".to_string()]
        ));
        // A value word from the wrong relation simply does not parse as a
        // value for this claim's relation.
        assert!(matches!(
            parse_claim("atlas continent arid"),
            Err(ParseError::NoValue)
        ));
        // "atlas" and "alps" both hit; "and" is a stopword, so ambiguity is
        // the refusal reason.
        assert!(matches!(
            parse_claim("atlas and alps continent europe"),
            Err(ParseError::Ambiguous("subject"))
        ));
    }

    #[test]
    fn authored_corpus_shape() {
        let events = events();
        assert_eq!(events.len(), 21);
        let mut slots: Vec<(usize, usize)> =
            events.iter().map(|f| (f.subject, f.relation)).collect();
        slots.dedup();
        assert_eq!(slots.len(), 20, "20 distinct slots plus one correction");
        assert_eq!(
            events[20],
            crate::claim::Fact {
                subject: 6,
                relation: 2,
                value: 1
            }
        );
    }
}
