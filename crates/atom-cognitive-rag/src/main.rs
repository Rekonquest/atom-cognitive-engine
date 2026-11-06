//! cognitive-rag: operator surface over one AtomDB store.

use atom_cognitive_rag::{CognitiveRag, Verdict};
use atom_db::Digest;
use std::{env, fs, process, str::FromStr};

const USAGE: &str = "usage:
  cognitive-rag <store> remember <source> <text...>
  cognitive-rag <store> remember-file <source> <file>
  cognitive-rag <store> ingest-jsonl <file>        (lines with \"source\" and \"text\")
  cognitive-rag <store> ask <question...>
  cognitive-rag <store> ask-json <question...>     (schema-1 packet for an LLM)
  cognitive-rag <store> feedback <passage-id> helpful|misleading";

fn main() {
    if let Err(error) = run() {
        eprintln!("cognitive-rag: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let [store, command, rest @ ..] = args.as_slice() else {
        return Err(USAGE.into());
    };
    match (command.as_str(), rest) {
        ("remember", [source, text @ ..]) if !text.is_empty() => {
            let mut rag = CognitiveRag::open(store)?;
            let passages = rag.remember(source, &text.join(" "))?;
            println!("remembered passages={}", passages.len());
        }
        ("remember-file", [source, file]) => {
            let mut rag = CognitiveRag::open(store)?;
            let passages = rag.remember(source, &fs::read_to_string(file)?)?;
            println!("remembered passages={}", passages.len());
        }
        ("ingest-jsonl", [file]) => {
            let mut rag = CognitiveRag::open(store)?;
            let (mut docs, mut passages) = (0, 0);
            for line in fs::read_to_string(file)?.lines() {
                let (Some(source), Some(text)) = (field(line, "source"), field(line, "text"))
                else {
                    continue;
                };
                passages += rag.remember(&source, &text)?.len();
                docs += 1;
            }
            println!("ingested documents={docs} passages={passages}");
        }
        ("ask", query) if !query.is_empty() => {
            let mut rag = CognitiveRag::open_read_only(store)?;
            print_answer(&rag.ask(&query.join(" "))?);
        }
        ("ask-json", query) if !query.is_empty() => {
            let mut rag = CognitiveRag::open_read_only(store)?;
            println!("{}", rag.ask(&query.join(" "))?.to_json());
        }
        ("feedback", [passage, polarity]) => {
            let helpful = match polarity.as_str() {
                "helpful" => true,
                "misleading" => false,
                _ => return Err(USAGE.into()),
            };
            let mut rag = CognitiveRag::open(store)?;
            let count = rag.feedback(Digest::from_str(passage)?, helpful)?;
            println!("{polarity} count={count}");
        }
        _ => return Err(USAGE.into()),
    }
    Ok(())
}

fn print_answer(answer: &atom_cognitive_rag::Answer) {
    println!("Q: {}", answer.query);
    match &answer.verdict {
        Verdict::Grounded { confidence } => println!("GROUNDED (confidence {confidence:.3})"),
        Verdict::Abstain(reason) => println!("ABSTAIN — {}", reason.describe()),
    }
    if let Some(fluency) = answer.fluency {
        println!("  {}", fluency.describe());
    }
    println!(
        "  shortlist {} passages → {} window traces, {} eligible",
        answer.shortlist, answer.window_traces, answer.eligible_traces
    );
    for c in &answer.citations {
        let excerpt: String = c.text.chars().take(160).collect();
        println!(
            "  [{:.4}] atomdb #{} engine {} — {} ({})",
            c.fused_score,
            c.atomdb_rank,
            c.engine_rank.map_or("gated".into(), |r| format!("#{r}")),
            c.sources.join(", "),
            &c.passage.to_hex()[..12],
        );
        println!("      {}", excerpt.replace('\n', " "));
    }
}

/// Flat JSON string-field extractor (handles \" escapes and \n).
fn field(line: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let rest = line.split(&needle).nth(1)?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start().strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                other => out.push(other),
            },
            '"' => return Some(out),
            c => out.push(c),
        }
    }
    None
}
