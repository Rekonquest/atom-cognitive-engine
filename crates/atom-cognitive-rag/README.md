# atom-cognitive-rag

Cognitive RAG: **AtomDB recalls, the cognitive engine judges.**

`rag-test/ask-atomdb.sh` proved the pattern with a shell pipe. This crate is
that hybrid as one in-process library and CLI, with durable feedback wired
into both halves.

```
question ──► AtomDB retrieval field ──► shortlist (cue intersections, exact threads)
                                             │
             feedback facts (AtomDB) ──► deliberation engine (fresh per ask)
                                             │  proximity-window traces, primed by
                                             │  replayed helpful / misleading facts
                                             ▼
                              gate → retrieve → assert-or-abstain
                                             │
                    reciprocal-rank fusion ◄─┘
                                             ▼
                  Answer { verdict, cited evidence, both lanes' ranks }
```

## Division of authority

| | AtomDB (long-term memory) | Cognitive engine (judgment) |
|---|---|---|
| Owns | passages, sources, feedback as immutable facts | gate, revision, abstention, dedup, fluency |
| Recall signal | how many query cues converge on a whole passage | whether the query holds together inside a 6-word window |
| Learning | Stage 9: feedback shifts conductance into a passage | replays the same facts as confirmations / contradictions |
| Writes | only `remember` and `feedback` | never; it lives for one ask |

Because the engine is rebuilt from durable facts on every ask, a verdict is a
deterministic function of the store plus the question (`response_hash` is
stable, and asking never changes `AtomDb::stats`).

## Verdicts

- `GROUNDED (confidence c)`: at least one shortlisted window passed the
  engine's gate; `c` is the mixture share of the "helpful" value lane.
  Only gated, not-net-misleading passages are cited.
- `ABSTAIN — no evidence`: AtomDB's cues converged on nothing.
- `ABSTAIN — evidence shares words but does not answer`: AtomDB found
  overlap, but no window holds the question together.
- `ABSTAIN — evidence is dominated by passages marked misleading`: the
  engine revised on feedback and the eligible mixture now says "misleading".

## Run

```bash
S=knowledge.atoms
cargo run -q -p atom-cognitive-rag -- $S remember observer-leases.md "A writer crash releases its lease. Lease recovery permits a replacement writer."
cargo run -q -p atom-cognitive-rag -- $S ingest-jsonl rag-test/atomdb-corpus/corpus.jsonl
cargo run -q -p atom-cognitive-rag -- $S ask how does writer lease recovery work after a crash
cargo run -q -p atom-cognitive-rag -- $S ask-json what are the eight cognitive roots   # LLM packet
cargo run -q -p atom-cognitive-rag -- $S feedback <passage-id> misleading
```

The store is an ordinary AtomDB file: the `atom-db` CLI can `retrieve`,
`get` and `verify` it too.

## Measured (121-passage corpus from `rag-test/atomdb-corpus`)

| Question | Verdict |
|---|---|
| what are the eight cognitive roots | GROUNDED, fluent; top citation is the engine project passage |
| what did the quantum attack resistance assessment conclude about Grover | GROUNDED via the assessment passage (AtomDB had it at #3) |
| the moon is made of cheese | ABSTAIN, shares words but does not answer (16 shortlisted, 0 eligible) |
| how do I bake sourdough bread | ABSTAIN, no evidence |

In the test fixture, AtomDB ranks a cue-dense decoy ("lease … crash …
writers … recovery" scattered through apartment advice) first; the engine
gates it out and fusion puts the real lease-recovery passage on top. Four
`misleading` marks on that passage flip the verdict to abstain, and the
marks survive reopening the store.

## Limits

- The engine-side vectors are hashed content words (128 dims), so the gate
  is lexical proximity, not semantics. Synonyms miss. `embed.rs` is the one
  place to swap in real embeddings (then raise `query_threshold`).
- `confidence` measures agreement about trustworthiness, not relevance; with
  no feedback yet it is 1.0 whenever anything passes the gate. Read it with
  `fluency` and `eligible_traces`.
- Feedback replays helpful marks before misleading ones per passage; the
  interleaving order is not preserved.
- No generation step. `ask-json` emits the context packet for an LLM.
- Depends on `Lucerna-Labs/atom-db` by pinned git rev (first-party, zero
  third-party dependencies are still the rule).
