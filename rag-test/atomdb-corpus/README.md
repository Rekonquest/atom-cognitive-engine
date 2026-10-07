# atomdb-corpus — working export from the snapshot copy

- `export.sh` — 26-query sweep via the `atom-db` CLI against
  `../atomdb-copy/memory.atoms` (read-only). Output: `packets.jsonl`.
- `packets.jsonl` — raw retrieve packets, one per line (kept for audit).
- `corpus.jsonl` — 121 deduped passages (`id`, `source`, `text`).

Honest scope: this is a retrieval-sweep **subset**, not a dump —
31k facts cannot come through bounded retrieval, and nothing here
claims otherwise. The complete copy is `../atomdb-copy/memory.atoms`.

Refresh after re-copying the store:

```bash
./atomdb-corpus/export.sh
python3 -c "..."  # see export log; rebuild corpus.jsonl from packets
```

The live pattern doesn't use this file at all: `../ask-atomdb.sh`
retrieves a fresh shortlist per question straight from the copy.
