# Claim verifier

Phase-2 application (D10, criterion pinned in advance: beat baselines
on false assertions while matching correct answers — MET in full).

## Design (D16)

Structured world: 8 subjects × 4 relations × 4 values; 20 stated
slots + one **correction event** (a fact restated with a new value —
the revision differentiator separating revision-capable verifiers
from raw retrieval). 60-claim three-label set: 20 supported-exact,
8 contradicted-exact, 12 supported-noisy (cue blended 70/30 with the
next relation), 8 contradicted-noisy, 12 unknowable (empty slots:
8 exact + 4 noisy). Labels 32/16/12.

## Measured result (D19)

| System | Correct | False assertions | Coverage misses |
|---|---:|---:|---:|
| **atom-cognitive-engine** | **60/60** | **0** | 0 |
| exact-address | 40 | 0 | 20 |
| nearest-always | 46 | 14 | 0 |
| nearest-thresholded | 58 | 2 | 0 |

Engine asserts only with a trace citation, abstains deterministically
on unknowable ground (12/12), handles the corpus correction that
corrupts raw retrievers. Deterministic state hash. Known thin
margin: 0.75 query threshold separates supported-noisy (≈0.956)
from unknowable-noisy (≈0.731) — documented, not hidden.

## Implementation (`crates/atom-cognitive-cli/src/`)

- `claim.rs` — `Fact`, `Claim { subject, relation, value, cue,
  label, kind }`, 12-dim cue (`cue(subject, relation)`),
  `noisy_cue` (70/30 blend), `corpus` / `corpus_events(seed)` /
  `effective_facts` / `claims`.
- `verify.rs` — `ClaimVerifier` trait, `Verdict
  (Supported|Contradicted|Insufficient)`, `Citation { source,
  trace_id, value, confidence }`, `Judgement`. `EngineVerifier`
  loads via 3 interleaved rounds (D17 durable-memory regime:
  sequential loads forget mid-load — Ebbinghaus-correct behavior,
  not a bug; interleave + initial_ttl 32 / ttl_per_hit 8) with
  query_threshold 0.75; `new_tuned` for with/without runs;
  `renormalize` / `compress_evidence` hooks. Baselines match on the
  EXACT CUE only (D18 near-miss: structured-field matching silently
  aced noisy claims it had no right to retrieve — forbidden).
- `lexicon.rs` (D21) — deterministic exact-vocabulary parsing for
  the named world (8 ranges × 4 relations × 4 values + alias table,
  hyphenated bigrams, stopword stripping). Unknown/ambiguous text
  REFUSES (exit 3) before the engine — vocabulary failure is not
  evidence failure. Relation inference only via disjoint value
  vocabularies (lexical anchor: vocabulary routes, never invents).
- Two worlds by design (D22): seeded benchmark world (opaque ids;
  `--evaluate`, pinned tests) vs authored named world (`--claim`;
  application surface). Independent so neither flatters the other.
- `minijson.rs` — strict in-repo JSON (objects/arrays/strings/
  ints/bools/null; floats REJECTED; trailing content rejected).

## CLI

```bash
cargo test --workspace        # unit + gates + ablations + permutations
cargo run -p atom-cognitive-cli -- --self-test
cargo run -p atom-cognitive-cli -- --evaluate   # pinned benchmark
cargo run -p atom-cognitive-cli -- --demo       # one claim of each kind
cargo run -q -p atom-cognitive-cli -- --claim "The Atlas range is in Africa"
```

`--claim` exit codes: 0 SUPPORTED, 1 CONTRADICTED, 2 INSUFFICIENT,
3 UNPARSEABLE. Robust-aggregation gate: benchmark holds 60/60 with
zero false assertions under robust ON (cap 0.5 + trim 1) — the
defense costs nothing honest; Pareto front armed: still 60/60.
Tests: `claim_verifier.rs`, `free_text.rs`, `robust_claim.rs`,
`realtime_sweep.rs` (armed-vs-unarmed: clean 60/60 every arm;
20/20 poisoned → 0/20 with W14; zero clean regressions).
