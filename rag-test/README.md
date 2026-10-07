# rag-test — sandboxed engine copy + AtomDB attachment

Deliberately separate from the main project: nothing here is part of
the real workspace, and nothing here changes it. A dated copy of
`atom-cognitive-core` + `atom-cognitive-engine`, a snapshot copy of
the live AtomDB store, and the bridge between them.

## Contents

- `crates/atom-cognitive-core`, `crates/atom-cognitive-engine` —
  verbatim copies (tests included) so experiments can't leak upstream.
- `crates/simple-rag` — the demo driver: word tokens (naive plural
  stemmer, stopwords stripped) + char trigrams, hashed into 128 dims
  and L2-normalized → one engine trace per sentence-chunk
  (title-prepended) via the record stack → queries return ranked
  chunks with mixture weights. No LLM, no generation — retrieval only.
  Takes `docs.txt` or a `.jsonl` corpus (`id`, `source`, `text`).
- `docs.txt` — 9 sample chunks (Rust, sourdough, houseplants).
  Swap in your own file with the same shape (`Title: ...` blocks
  separated by `---`).
- `atomdb-copy/` — exact snapshot of the live AtomDB store
  (`memory.atoms` + `MANIFEST.md` with SHA-256 and counts).
  Read-only; refresh by re-copying, never write.
- `atomdb-corpus/` — 121-passage sweep export (`export.sh`,
  `packets.jsonl`, `corpus.jsonl`, `README.md`). A subset for
  offline tests; the live pattern doesn't need it.
- `ask-atomdb.sh` — **the attachment**: AtomDB recall (shortlist via
  the `atom-db` CLI on the snapshot copy) + engine judgment (traces
  with AtomDB passage ids as citations, assert or abstain).
- `target-atomdb/` — build output of the AtomDB source snapshot's
  `atom-db-cli` (binary `atom-db`: `stats`/`retrieve`/`get`,
  read-only). Built from `/home/jesse/AtomDB-MCP/source`, target
  dir kept inside this sandbox.

## Run (from this folder)

```bash
cargo run -q -p simple-rag                    # ingests docs.txt, runs 4 demo queries
cargo run -q -p simple-rag -- docs.txt how do I feed my starter
cargo run -q -p simple-rag -- atomdb-corpus/corpus.jsonl <query>
./ask-atomdb.sh "what are the eight cognitive roots"
```

The first arg (if any) is the docs file; the rest is one query.
Top-3 chunks print with weights + trace ids; empty retrieval
abstains instead of guessing.

## Regime notes

- `conservation_budget: 1e6`, ttl 512, tiny decay: a retrieval
  store must not eat itself — the main project's probe lesson
  (budget 12 vs 54 traces) applies here in reverse.
- `query_threshold: 0.5` (the floor): hashed word-vectors are
  coarse, so the gate stays permissive; raise it if you add real
  embeddings.
