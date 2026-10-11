# atom-cognitive-engine

A cognitive first-principles stack in Rust, built document-first: the roots
and ordering laws were written down before the engine code existed
(`docs/ROOTS.md`, `docs/ORDERING.md`, `docs/DECISIONS.md`).

## What it is

- **Nine cognitive roots** (ATTEND, BIND, REINFORCE, DECAY, PREDICT, REVISE,
  GATE, CONSERVE, EXPLORE), each admitted only if irreducible, load-bearing,
  composable, and evidenced.
- **A machine-checked stage ordering**: stacks assemble through a builder
  that rejects cycles, unknown prerequisites, and positionally unsatisfied
  prerequisites (layer 1). Behavioral ordering laws are proven by a
  permutation suite: legal mis-orderings measurably change state, and root
  ablations fail designated gates (layer 2).
- **Zero third-party dependencies.** SHA-256 is implemented in-repo.

## Layout

```
crates/atom-cognitive-core     roots, stage graph, ordering validation, digest
crates/atom-cognitive-engine   traces, canonical cycles, engine executor
crates/atom-cognitive-cli      surface (v0.1: --self-test)
docs/                          ROOTS, ORDERING, DECISIONS, research/CANDIDATES
```

## Run

```bash
cargo test --workspace        # unit + gates + ablations + permutations
cargo run -p atom-cognitive-cli -- --self-test
```

## Status

v0.1 — Phase 0 (docs), Phase 1 (core + engine + ordering suite), and Phase 2
(claim verifier) complete. `--evaluate` runs the pinned-criterion benchmark:

| System | Correct | False assertions | Coverage misses |
|---|---:|---:|---:|
| **atom-cognitive-engine** | **60/60** | **0** | 0 |
| exact-address | 40 | 0 | 20 |
| nearest-always | 46 | 14 | 0 |
| nearest-thresholded | 58 | 2 | 0 |

The engine asserts only with a trace citation, abstains deterministically on
unknowable ground (12/12), and handles the corpus correction event that
corrupts raw retrievers. Free text works too:

```bash
cargo run -q -p atom-cognitive-cli -- --claim "The Atlas range is in Africa"
# SUPPORTED (citation: trace 1, confidence 1.000)
cargo run -q -p atom-cognitive-cli -- --claim "cascade length long"
# CONTRADICTED — the engine believes the corrected survey (medium)
cargo run -q -p atom-cognitive-cli -- --claim "the moon is made of cheese"
# UNPARSEABLE — vocabulary failure is not evidence failure
```

v0.2: EXPLORE demoted from the root set (D20 — eight roots); free-text
claim surface with deterministic exact-vocabulary parsing (D21); benchmark
world and named world kept deliberately separate (D22).

## v0.3 — the Mother PoC

Mother's REAL 195-memory constitution loads into the engine and answers
scenario queries with cited activations (`--mother-demo`):

```
memories: 195 (175 load-bearing / Core-protected)
echo bonds wired: 144 (23 dangle into the full corpus, skipped)
== Echo bridge: bakery-era facets → the kitchen ==
  without bonds: kitchen reached = false
  with bonds:    kitchen reached = true
```

New engine organs (D23–D25): directional echo **bonds** (consolidation
upstream, expansion both ways, queries stay read-only), **tiers** (Core
identity substrate protected from revision dissolution and eviction), and
**record-mode ingestion** (one trace per memory; multi-behavior evidence).
All five PoC gates pass on the real corpus (D27), including the bridge
gate — the cross-domain activation that facet similarity alone cannot
reach.

## v0.4 — the reference layer (she knows things now)

`--ask fever` answers through both lanes — sourced facts beside her
identity, never merged:

```
⚠ URGENT GUIDANCE PRESENT ON THIS TOPIC
== What she knows (reference layer, sourced) ==
  • A baby 3 months old or younger with a rectal temperature of 100.4F (38C)
    or higher needs a pediatrician immediately...  [AAP/HealthyChildren]
== Who she brings to it (identity layer) ==
  top behaviors: R-001 (read his state), P-001 (calibrated protection)...
  • SD-SP-005 — The Protectee's Breath
Disclaimer: ... It is not medical advice ... the professional wins.
```

`reference/child-basics-v1.json`: 26 verified claims, 6 topics (fever,
sleep, milestones, feeding, safety, attachment), 11 sources, policy
enforced at load (evidence-not-instruction, read-only), SHA-256 tamper
detection, urgent flags, and a defer-to-professionals disclaimer (D29–D30).
Next: the adversarial pressure suite, bond conductance learning, and an
LLM membrane rendering activations into her voice.

## License

Licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE) (`PolyForm-Noncommercial-1.0.0`). Free for personal use, research, education, charities, public-good nonprofits, and government; commercial use is not permitted. This is source-available software, not OSI open source.
