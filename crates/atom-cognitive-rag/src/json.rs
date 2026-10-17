//! Schema-1 JSON for an answer: the LLM-facing context packet. Hand-written
//! (no serde) to keep the zero-third-party-dependency law.

use crate::{Answer, Verdict};
use std::fmt::Write;

pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

fn strings(items: &[String]) -> String {
    let quoted: Vec<String> = items.iter().map(|s| format!("\"{}\"", escape(s))).collect();
    format!("[{}]", quoted.join(","))
}

impl Answer {
    pub fn to_json(&self) -> String {
        let (verdict, confidence, reason) = match &self.verdict {
            Verdict::Grounded { confidence } => {
                ("grounded", format!("{confidence:.6}"), "null".into())
            }
            Verdict::Abstain(reason) => (
                "abstain",
                "null".to_string(),
                format!("\"{}\"", reason.describe()),
            ),
        };
        let mut out = String::new();
        let _ = write!(
            out,
            "{{\"schema\":1,\"query\":\"{}\",\"verdict\":\"{verdict}\",\"confidence\":{confidence},\
             \"abstain_reason\":{reason},\"fluency\":{},\"shortlist\":{},\"window_traces\":{},\
             \"eligible_traces\":{},\"snapshot_sequence\":{},\"response_hash\":\"{}\",\"citations\":[",
            escape(&self.query),
            self.fluency
                .map_or("null".into(), |f| format!("\"{}\"", escape(f.describe()))),
            self.shortlist,
            self.window_traces,
            self.eligible_traces,
            self.snapshot_sequence,
            self.response_hash,
        );
        for (i, c) in self.citations.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "{{\"passage\":\"{}\",\"sources\":{},\"fused_score\":{:.6},\"atomdb_rank\":{},\
                 \"atomdb_activation\":{},\"supporting_cues\":{},\"engine_rank\":{},\
                 \"engine_weight\":{:.6},\"feedback\":{{\"helpful\":{},\"misleading\":{}}},\"text\":\"{}\"}}",
                c.passage,
                strings(&c.sources),
                c.fused_score,
                c.atomdb_rank,
                c.atomdb_activation,
                strings(&c.supporting_cues),
                c.engine_rank.map_or("null".into(), |r| r.to_string()),
                c.engine_weight,
                c.feedback.0,
                c.feedback.1,
                escape(&c.text),
            );
        }
        out.push_str("]}");
        out
    }
}
