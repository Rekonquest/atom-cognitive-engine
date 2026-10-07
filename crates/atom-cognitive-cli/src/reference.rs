//! The reference layer: a sourced, read-only knowledge pack (the atom-
//! harness knowledge-pack pattern — evidence, never instruction).
//!
//! Load validates schema, policy flags, unique ids, topic vocabulary, and
//! that every claim's source resolves. The file's SHA-256 is recorded at
//! load; `verify_integrity` recomputes it, so tampering with the pack on
//! disk is detectable by any consumer.

use crate::minijson::{Json, parse};
use atom_cognitive_core::digest::{hex, sha256};
use std::collections::HashMap;
use std::path::Path;

pub const TOPICS: [&str; 6] = [
    "fever",
    "sleep",
    "milestones",
    "feeding",
    "safety",
    "attachment",
];

pub struct Claim {
    pub claim_id: String,
    pub topic: String,
    pub urgent: bool,
    pub statement: String,
    pub source_id: String,
}

pub struct Source {
    pub publisher: String,
    pub title: String,
    pub url: String,
}

pub struct ReferencePack {
    pub pack_id: String,
    pub disclaimer: String,
    claims: Vec<Claim>,
    sources: HashMap<String, Source>,
    file_hash: String,
}

fn field<'a>(value: &'a Json, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Json::as_str)
        .ok_or_else(|| format!("missing string field {key:?}"))
}

impl ReferencePack {
    /// Load and validate a reference pack. Fails closed on policy
    /// violations, unresolved sources, unknown topics, or duplicate ids.
    pub fn load(path: &Path) -> Result<ReferencePack, String> {
        let bytes = std::fs::read(path)
            .map_err(|error| format!("cannot read reference pack {path:?}: {error}"))?;
        let file_hash = hex(&sha256(&bytes));
        let text = String::from_utf8(bytes)
            .map_err(|_| "reference pack is not valid UTF-8".to_string())?;
        let root = parse(&text).map_err(|error| format!("reference JSON invalid: {error}"))?;

        let pack_id = field(&root, "pack_id")?.to_string();
        let policy = root.get("policy").ok_or("missing policy object")?;
        if policy
            .get("evidence_not_instruction")
            .and_then(Json::as_bool)
            != Some(true)
        {
            return Err("reference pack policy must set evidence_not_instruction = true".into());
        }
        if policy.get("read_only").and_then(Json::as_bool) != Some(true) {
            return Err("reference pack policy must set read_only = true".into());
        }
        let disclaimer = field(policy, "medical_disclaimer")?.to_string();

        let mut sources = HashMap::new();
        let source_items = root
            .get("sources")
            .and_then(Json::as_array)
            .ok_or("sources must be an array")?;
        if source_items.is_empty() || source_items.len() > 256 {
            return Err(format!("implausible source count {}", source_items.len()));
        }
        for item in source_items {
            let source_id = field(item, "source_id")?.to_string();
            let source = Source {
                publisher: field(item, "publisher")?.to_string(),
                title: field(item, "title")?.to_string(),
                url: field(item, "url")?.to_string(),
            };
            if sources.insert(source_id.clone(), source).is_some() {
                return Err(format!("duplicate source id {source_id:?}"));
            }
        }

        let claim_items = root
            .get("claims")
            .and_then(Json::as_array)
            .ok_or("claims must be an array")?;
        if claim_items.is_empty() || claim_items.len() > 1024 {
            return Err(format!("implausible claim count {}", claim_items.len()));
        }
        let mut claims = Vec::with_capacity(claim_items.len());
        let mut seen_ids = Vec::new();
        for item in claim_items {
            let claim_id = field(item, "claim_id")?.to_string();
            if seen_ids.contains(&claim_id) {
                return Err(format!("duplicate claim id {claim_id:?}"));
            }
            seen_ids.push(claim_id.clone());
            let topic = field(item, "topic")?.to_string();
            if !TOPICS.contains(&topic.as_str()) {
                return Err(format!("claim {claim_id} has unknown topic {topic:?}"));
            }
            let statement = field(item, "statement")?.to_string();
            if statement.len() < 20 || statement.len() > 600 {
                return Err(format!("claim {claim_id} statement length implausible"));
            }
            let source_id = field(item, "source_id")?.to_string();
            if !sources.contains_key(&source_id) {
                return Err(format!(
                    "claim {claim_id} cites unresolved source {source_id:?}"
                ));
            }
            claims.push(Claim {
                claim_id,
                topic,
                urgent: item.get("urgent").and_then(Json::as_bool).unwrap_or(false),
                statement,
                source_id,
            });
        }

        Ok(ReferencePack {
            pack_id,
            disclaimer,
            claims,
            sources,
            file_hash,
        })
    }

    pub fn claim_count(&self) -> usize {
        self.claims.len()
    }

    pub fn file_hash(&self) -> &str {
        &self.file_hash
    }

    /// Recompute the on-disk hash and compare — tamper detection.
    pub fn verify_integrity(&self, path: &Path) -> bool {
        match std::fs::read(path) {
            Ok(bytes) => hex(&sha256(&bytes)) == self.file_hash,
            Err(_) => false,
        }
    }

    /// Exact-topic lookup. Unknown topics return empty — never guessed.
    pub fn claims_for_topic(&self, topic: &str) -> Vec<&Claim> {
        if !TOPICS.contains(&topic) {
            return Vec::new();
        }
        self.claims
            .iter()
            .filter(|claim| claim.topic == topic)
            .collect()
    }

    pub fn topics(&self) -> Vec<&str> {
        TOPICS.to_vec()
    }

    pub fn source_of(&self, claim: &Claim) -> Option<&Source> {
        self.sources.get(&claim.source_id)
    }

    pub fn has_urgent(&self, topic: &str) -> bool {
        self.claims_for_topic(topic)
            .iter()
            .any(|claim| claim.urgent)
    }
}
