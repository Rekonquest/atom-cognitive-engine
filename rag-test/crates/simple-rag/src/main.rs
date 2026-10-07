//! simple-rag: the smallest honest RAG over a copy of the cognitive engine.
//!
//! Pipeline: docs.txt chunks -> hashed bag-of-words cue vectors -> one
//! engine trace per chunk (record stack) -> query embeds the question and
//! the engine retrieves ranked chunks with mixture weights as citations.
//! No LLM, no generation step: retrieval is the whole demo.

use atom_cognitive_engine::stacks::{query_stack, record_stack};
use atom_cognitive_engine::{CognitiveEngine, EngineConfig};
use std::collections::HashMap;

const DIM: usize = 128;
const WORD_DIMS: usize = 48;

const STOPWORDS: &[&str] = &[
    "the", "and", "with", "from", "that", "this", "have", "has", "are", "was", "were", "for",
    "you", "your", "its", "into", "over", "under", "then", "than", "when", "what", "which",
    "while", "also", "always", "never", "every", "without", "through", "before", "after",
];

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Naive demo stemmer: strip a trailing 's' so plant/plants match.
fn stem(token: &str) -> &str {
    if token.len() > 4 && token.ends_with('s') && !token.ends_with("ss") {
        &token[..token.len() - 1]
    } else {
        token
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() > 2 && !STOPWORDS.contains(t))
        .map(|t| stem(t).to_string())
        .collect()
}

/// Word tokens (first WORD_DIMS dims) + char trigrams (rest), L2-normalized.
/// Trigrams forgive morphology the stemmer misses (houseplant/houseplants).
fn embed(text: &str) -> Vec<f64> {
    let mut vec = vec![0.0f64; DIM];
    for token in tokenize(text) {
        vec[(fnv1a(token.as_bytes()) % WORD_DIMS as u64) as usize] += 1.0;
    }
    let padded = format!(" {text} ").to_lowercase();
    let chars: Vec<char> = padded.chars().collect();
    for window in chars.windows(3) {
        let gram: String = window.iter().collect();
        let bytes = gram.as_bytes();
        vec[WORD_DIMS + (fnv1a(bytes) % (DIM - WORD_DIMS) as u64) as usize] += 1.0;
    }
    let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norm > f64::EPSILON {
        for v in &mut vec {
            *v /= norm;
        }
    }
    vec
}

struct Chunk {
    title: String,
    body: String,
    /// AtomDB passage id, or `local-N` for docs.txt chunks.
    pid: String,
    source: String,
}

/// Minimal JSON string-field extractor (flat objects only; handles \" escapes).
fn json_field(line: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let mut rest = line.split(&needle).nth(1)?.trim_start();
    rest = rest.strip_prefix(':')?.trim_start().strip_prefix('"')?;
    let mut out = String::new();
    let mut escaped = false;
    for c in rest.chars() {
        if escaped {
            out.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
}

fn split_sentences(title: &str, text: &str, pid: &str, source: &str, chunks: &mut Vec<Chunk>) {
    for sentence in text.split(['.', '?', '!']) {
        let sentence = sentence.trim();
        if sentence.split_whitespace().count() >= 4 {
            chunks.push(Chunk {
                title: title.to_string(),
                body: format!("{title}. {sentence}"),
                pid: pid.to_string(),
                source: source.to_string(),
            });
        }
    }
}

/// AtomDB export: one JSONL line per passage, sentence-chunked, ids kept.
fn load_jsonl(path: &str) -> Vec<Chunk> {
    let text = std::fs::read_to_string(path).expect("read corpus file");
    let mut chunks = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let (Some(pid), Some(source), Some(body)) = (
            json_field(line, "id"),
            json_field(line, "source"),
            json_field(line, "text"),
        ) else {
            continue;
        };
        let title = format!("{} [{}]", source, &pid[..pid.len().min(12)]);
        split_sentences(&title, &body, &pid, &source, &mut chunks);
    }
    chunks
}

fn load_docs(path: &str) -> Vec<Chunk> {
    if path.ends_with(".jsonl") {
        return load_jsonl(path);
    }
    let text = std::fs::read_to_string(path).expect("read docs file");
    let mut chunks = Vec::new();
    let mut local = 0usize;
    for block in text.split("\n---\n").filter(|b| !b.trim().is_empty()) {
        let block = block.trim();
        let mut lines = block.lines();
        let title = lines
            .next()
            .unwrap_or("untitled")
            .strip_prefix("Title:")
            .unwrap_or("untitled")
            .trim()
            .to_string();
        // One chunk per sentence, title-prepended (standard tiny-RAG
        // practice: keeps vectors tight and every chunk self-describing).
        let body = block
            .lines()
            .filter(|l| !l.trim().starts_with("Title:"))
            .collect::<Vec<_>>()
            .join(" ");
        let pid = format!("local-{local}");
        local += 1;
        split_sentences(&title, &body, &pid, "docs.txt", &mut chunks);
    }
    chunks
}

fn ask(engine: &CognitiveEngine, chunks: &[Chunk], chunk_of: &HashMap<u64, usize>, query: &str) {
    let outcome = engine
        .query(&query_stack(), &embed(query))
        .expect("query runs");
    println!("Q: {query}");
    if outcome.activations.is_empty() {
        println!("  (abstained — nothing relevant in store)");
        return;
    }
    for (trace_id, weight) in outcome.activations.iter().take(3) {
        match chunk_of.get(trace_id) {
            Some(i) => println!(
                "  [{weight:.3}] {} (passage {}, trace {trace_id})",
                chunks[*i].title, chunks[*i].pid
            ),
            None => println!("  [{weight:.3}] ? (trace {trace_id})"),
        }
    }
    match outcome.claim {
        Some((value, confidence)) => println!("  engine value={value} confidence={confidence:.3}"),
        None => println!("  engine abstained on value, chunks still cited above"),
    }
}

fn main() {
    let docs_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs.txt".to_string());
    let docs = load_docs(&docs_path);
    assert!(docs.len() >= 2, "need at least two chunks");

    // Retrieval regime, not a memory experiment: budget far above the
    // load so conservation never eats the store, long ttl, tiny decay.
    let config = EngineConfig {
        cue_dim: DIM,
        value_count: 8,
        capacity: 1024,
        initial_ttl: 512,
        ttl_per_hit: 16,
        decay_support_rate: 0.0005,
        decay_mass_rate: 0.0003,
        conservation_budget: 1.0e6,
        query_threshold: 0.5,
        ..EngineConfig::default()
    };
    config.validate().expect("config valid");
    let mut engine = CognitiveEngine::new(config).expect("engine starts");

    let record = record_stack();
    let mut chunk_of: HashMap<u64, usize> = HashMap::new();
    for (index, chunk) in docs.iter().enumerate() {
        engine
            .observe(&record, &embed(&chunk.body), index % 8, 1.0)
            .expect("ingest chunk");
        let trace_id = engine.traces().last().expect("nucleated").id;
        chunk_of.insert(trace_id, index);
    }
    println!(
        "ingested {} chunks into {} traces",
        docs.len(),
        engine.active_traces().len()
    );

    let rest: Vec<String> = std::env::args().skip(2).collect();
    if rest.is_empty() {
        for demo in [
            "how do Rust references work",
            "how do I get a thick crust on sourdough",
            "why is my houseplant leaning toward the window",
            "how to water indoor plants without killing them",
        ] {
            ask(&engine, &docs, &chunk_of, demo);
            println!();
        }
    } else {
        ask(&engine, &docs, &chunk_of, &rest.join(" "));
    }
}
