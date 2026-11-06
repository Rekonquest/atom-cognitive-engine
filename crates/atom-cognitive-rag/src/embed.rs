//! The engine-side cue transducer: deterministic hashed content-word vectors
//! over short proximity windows (descended from `rag-test/simple-rag`; the
//! char trigrams were dropped because their background overlap let unrelated
//! text through the gate).
//!
//! This is deliberately coarse. It never decides recall on its own: AtomDB's
//! exact multi-cue field picks the shortlist, and these vectors only let the
//! engine's gate judge how close each shortlisted window sits to the
//! question. Swap in real embeddings here and raise `query_threshold`.

pub const DIM: usize = 128;
/// Content words per proximity window, and the stride between windows.
pub const WINDOW: usize = 6;
const STRIDE: usize = 3;

const STOPWORDS: &[&str] = &[
    "the", "and", "with", "from", "that", "this", "have", "has", "are", "was", "were", "for",
    "you", "your", "its", "into", "over", "under", "then", "than", "when", "what", "which",
    "while", "also", "always", "never", "every", "without", "through", "before", "after", "how",
    "does", "why", "who", "where", "can", "not", "but", "all", "any", "our", "out", "about",
    "there", "their", "them", "they", "will", "would", "should", "could", "been", "being", "is",
    "made", "make", "work", "works", "use", "used", "using",
];

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Light suffix stemmer. Over-stemming is harmless here: queries and
/// passages go through the same function.
fn stem(token: &str) -> String {
    let mut t = token;
    for suffix in ["ing", "ion", "ed", "ly"] {
        if t.len() > suffix.len() + 3 && t.ends_with(suffix) {
            t = &t[..t.len() - suffix.len()];
            break;
        }
    }
    if let Some(base) = t.strip_suffix("ery").filter(|b| b.len() > 3) {
        return format!("{base}er");
    }
    if t.len() > 4 && t.ends_with('s') && !t.ends_with("ss") {
        t = &t[..t.len() - 1];
    }
    t.to_string()
}

/// Content words: lowercase alphanumeric runs, stopwords dropped, stemmed.
pub fn content_words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() > 2 && !STOPWORDS.contains(t))
        .map(stem)
        .collect()
}

/// Hashed bag of content words, L2-normalized. `None` when nothing is left
/// (the engine rejects all-zero cues).
pub fn embed_words(words: &[String]) -> Option<Vec<f64>> {
    let mut vec = vec![0.0f64; DIM];
    for word in words {
        vec[(fnv1a(word.as_bytes()) % DIM as u64) as usize] = 1.0;
    }
    let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norm <= f64::EPSILON {
        return None;
    }
    for v in &mut vec {
        *v /= norm;
    }
    Some(vec)
}

pub fn embed(text: &str) -> Option<Vec<f64>> {
    embed_words(&content_words(text))
}

/// Proximity windows over a passage's content words (WINDOW words, STRIDE
/// apart). A window's cosine with the query clears the engine's 0.5 gate
/// only when roughly half the query's content words co-occur inside it,
/// so the engine judges *local* coherence where AtomDB counted cues across
/// the whole passage.
pub fn windows(passage: &str) -> Vec<Vec<String>> {
    let words = content_words(passage);
    if words.len() <= WINDOW {
        return if words.is_empty() {
            Vec::new()
        } else {
            vec![words]
        };
    }
    let mut out = Vec::new();
    let mut start = 0;
    loop {
        let end = (start + WINDOW).min(words.len());
        out.push(words[start..end].to_vec());
        if end == words.len() {
            break;
        }
        start += STRIDE;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedding_is_unit_and_deterministic() {
        let a = embed("Writer lease recovery after a crash").unwrap();
        let b = embed("Writer lease recovery after a crash").unwrap();
        assert_eq!(a, b);
        let norm: f64 = a.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-9);
    }

    #[test]
    fn windows_cover_the_passage() {
        assert_eq!(windows("tiny words here").len(), 1);
        let w = windows("alpha bravo charlie delta echo foxtrot golf hotel india juliet");
        assert_eq!(w.len(), 3);
        assert_eq!(w.last().unwrap().last().unwrap(), "juliet");
    }

    #[test]
    fn proximity_gate_separates_coherent_windows() {
        let query = embed("writer lease recovery crash").unwrap();
        let hit = embed("a crash releases the writer lease; recovery is immediate").unwrap();
        let miss = embed("sourdough starter needs flour water and patience daily").unwrap();
        let cos = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>();
        assert!(cos(&query, &hit) >= 0.5);
        assert!(cos(&query, &miss) < 0.5);
    }
}
