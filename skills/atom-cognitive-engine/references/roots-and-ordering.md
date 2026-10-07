# Roots and ordering

## Admission filter (a ROOT only if all four hold)

1. **Irreducible** — other candidates decompose into compositions of
   roots, never the reverse.
2. **Load-bearing** — removing it breaks a measurable behavior (a gate
   fails).
3. **Composable** — participates in ordered, acyclic compositions.
4. **Evidenced** — grounded in receipts or cited literature.

Failing candidates become compositions, are demoted, or stay out of
scope. REVISE keeps root status despite decomposing to
DECAY→BIND→REINFORCE because its trigger (contradiction) and internal
law (dissolve-before-replace) are independently load-bearing (D5).

## Composition table (candidates → root compositions)

| Candidate | Composition |
|---|---|
| remember | DECAY → BIND → REINFORCE ("apply time, then reshape") |
| learn | ATTEND → (BIND \| REINFORCE) → CONSERVE, attention-first |
| retrieve | ATTEND → GATE → mixture → assert-or-abstain |
| forget | DECAY (+ CONSERVE eviction) |
| abstract | repetition/consolidation → BIND shared structure → DECAY instances |
| consolidate | offline: PREDICT → low-surprise clusters → abstract |
| curiosity | formation-regime variation from PREDICT error entropy (mechanism, not root) |
| interference | failure mode governed by GATE, not an operation |
| homeostasis | PREDICT + CONSERVE at the system boundary (governor lane, out of scope) |

## Canonical cycles

- **Observe (write)**: validate → PREDICT (expectation + surprise) →
  DECAY (time passes first) → ATTEND → GATE (bind-or-nucleate fork) →
  BIND/REINFORCE/REVISE (the write) → CONSERVE (budget last) → verify.
- **Query (read)**: validate → ATTEND → GATE (eligibility) → mixture →
  assert-or-abstain → verify. Never assert before GATE.
- **Consolidation (offline)**: PREDICT over stored state → cluster by
  low surprise → BIND shared structure → DECAY instances → CONSERVE
  → verify.

## Law table (O1–O10)

| # | Law | Enforcement | Receipt |
|---|---|---|---|
| O1 | Time passes before learning (DECAY before BIND) | behavioral (permutation: learn-before-forget fails revision/retention) | C.1 |
| O2 | Order/structure exists before ATTEND consumes it | machine-checked (structure-validation is ATTEND's prerequisite) | C.2 |
| O3 | GATE before blend/route-merge; admission ≠ license | behavioral + machine-checked (eligibility precedes assertion) | C.3 |
| O4 | Load (state, capabilities) before traffic (queries) | machine-checked (preload precedes query stages) | C.4 |
| O5 | Compositions are acyclic | machine-checked (builder rejects cycles) | C.6 |
| O6 | Writes budgeted AFTER the write (CONSERVE last) | behavioral (conserve-before-write fails capacity) | C.1 |
| O7 | Never assert before retrieval + eligibility | machine-checked (assert requires retrieve+gate) + behavioral | C.3 |
| O8 | Exploration decays monotonically; settle before assert | behavioral (no-cooling variant fails settle gates) | chaos-budget |
| O9 | Verify at boundaries; reordered state rejected | machine-checked (hash verification at cycle ends) | C.7 |
| O10 | Weaken harsher than strengthen | invariant in `EngineConfig::validate` | C.8 |

## Stacks (`atom-cognitive-engine/src/stacks.rs`)

`observe_stack`, `query_stack`, `idle_stack` (canonical);
`record_stack` (gate always nucleates — corpus/constitution
ingestion, one trace per memory); `kpr_stack` (candidate gate +
promote stage); `mother_query_stack` (query + `query.expand` bond
BFS between gate and retrieval); `misordered_decay_after_write`
(legal O1 permutation for tests); `ungated_query_stack`
(deliberately reachable O7 failure mode — proves GATE load-bearing);
`omit(stack, stage)` (ablation helper, drops dependents
transitively). Prerequisites encode layer-1 safety only; behavioral
laws stay assemblable-around so permutation tests can prove them.

## With/without audit (D42) — which laws are permanent vs experimental

- EARNED (measured failure receipts — keep): forget-before-learn,
  order-before-attention, gate-before-blend, preload-before-traffic,
  acyclic compositions.
- ADMISSION-GATED (failing-ablation proofs — keep): KPR staging,
  tiers, bond depth, expansion reliability floor.
- INHERITED TASTE (no local receipt — ablation candidates, each
  gets a with/without experiment): read-only-queries (Stage-9 —
  the testing-effect literature contradicts it directly),
  O10 weaken-harsher, two-lane separation, the 0.75 verifier
  threshold (thin margin, documented not hidden).

The control family — hysteresis (#3), canalization (#11), integral
adaptation (#12) — is DEFERRED per D40 (operator directive:
discovery before controls; a stabilizer chosen before visible
instability is a guess). Watchlist that earns them: tier/law
chattering, persona mode thrash, route flapping,
drift-without-correction.
