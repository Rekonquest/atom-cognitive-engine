# atomdb-copy — snapshot manifest

Exact file-level copy of the live AtomDB store, taken 2026-09-21.
The copy is complete (every atom, bond, cell); the engine cannot
parse `.atoms` directly, so the *working* attachment is the
JSONL export in `../atomdb-corpus/` (passages with AtomDB ids).
This file copy is the ground truth behind that export.

- Source: `/home/jesse/AtomDB-MCP/data/memory.atoms`
- Copy: `rag-test/atomdb-copy/memory.atoms`
- SHA-256 (both, must match): `277da04fd14a0809534e316ba97c94184060bd0728d7e1853f9c012193428112`
- Snapshot sequence: 32755
- Contents at copy time: 5768 atoms, 26744 bonds, 120 cells,
  32515 facts, 32755 frames, store healthy, 5902306 durable bytes
- AtomDB version: 0.2.0 (`/home/jesse/AtomDB-MCP/VERSION.txt`)

Refresh (never write to the live store from here):

```bash
cp /home/jesse/AtomDB-MCP/data/memory.atoms rag-test/atomdb-copy/memory.atoms
```

then re-run the export sweep (see `../atomdb-corpus/README.md`).
All export reads against the copy open read-only.
