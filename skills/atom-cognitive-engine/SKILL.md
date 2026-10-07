---
name: atom-cognitive-engine
description: Build, extend, or audit the atom-cognitive-engine Rust workspace — its eight cognitive roots, machine-checked stage ordering, observe/query/idle cycles, engine API, claim-verifier benchmark, and with/without gate discipline. Use when the task touches crates/atom-cognitive-core, crates/atom-cognitive-engine, stacks, EngineConfig, traces, bonds, tiers, KPR, or the CLI claim surface. Do not route Lightcone geometry, Mother-constitution content, or reference-pack facts here — see atom-mother for those.
---

# atom-cognitive-engine

A cognitive first-principles stack in Rust: document-first (doctrine in
`docs/` outranks code), zero third-party dependencies (SHA-256 is
in-repo), `unsafe_code = "forbid"`, edition 2024. The engine executes
whatever ordering-validated stack it is handed — mis-ordered but legal
stacks genuinely execute in that order, which is how the permutation
suite proves the ordering laws.

## Layout

```text
crates/atom-cognitive-core     roots, stage graph, ordering validation, digest
crates/atom-cognitive-engine   traces, canonical cycles, engine executor
crates/atom-cognitive-cli      surface: claim verifier, lexicon, Mother, persona, reference
docs/                          ROOTS.md, ORDERING.md, DECISIONS.md, research/
reference/child-basics-v1.json sourced knowledge pack (see atom-mother)
```

## The eight roots (docs/ROOTS.md, v0.2)

ATTEND (selection weights, never a decision), BIND (nucleation of new
links), REINFORCE (strengthen with use, rate bounded), DECAY (weaken
with time/disuse/contradiction; retire at floors), PREDICT
(expectation + surprise error signal), REVISE (contradiction
resolution: dissolve-first-then-replace, forget-before-learn),
GATE (hard pass/block/redirect — where ATTEND weights, GATE decides),
CONSERVE (renormalize onto budgets after writes; weakest-first
eviction). EXPLORE was admitted v0.1 and **demoted v0.2 (D20)** — its
mechanisms (seeded tie-breaks, monotonic cooling) are behaviorally
inert; the code remains as invariant-tested mechanism claiming no root
status. Every root must have an ablation gate that fails without it
(D9); a root with no failing ablation is demoted.

## Ordering laws (docs/ORDERING.md)

Two enforcement layers. (1) **Machine-checked prerequisites**: stages
declare prerequisites; `Stack::assemble` rejects duplicates, unknown
prerequisites, positionally unsatisfied prerequisites, cycles, and
empty stacks — violating these is a construction error, the stack
does not exist. (2) **Behavioral order laws** (O1–O10): legal to
assemble but measurably worse; each carries a permutation test proving
a designated gate fails under the mis-order. Canonical orders:

- observe: `validate → predict → decay → attend → gate → write → conserve → verify`
- query: `validate → attend → gate → retrieve → assert_or_abstain → verify`
- consolidate: `validate → predict_cluster → bind_shared → decay_instances → conserve → verify`

`learn` is attention-first: `ATTEND → (BIND | REINFORCE) → CONSERVE`.
O10 (weaken harsher than strengthen) is an arithmetic invariant in
`EngineConfig::validate`: `(1 − contradiction_support_factor)` must
exceed `reinforcement_support`. Defaults: reinforce +0.12 support,
contradiction factor 0.82 (cost 0.18 > gain 0.12).

## Engine essentials

- `CognitiveEngine::new(EngineConfig)` — fail-closed validation.
- `observe(&observe_stack, cue, value, salience)` → nucleate / bind /
  reinforce / revise. Surprise (predicted ≠ observed) boosts salience
  ×1.5 (PREDICT is load-bearing through this — ablation-gated).
- `query(&query_stack, cue)` → `QueryOutcome { claim: Option<(value,
  confidence)>, activations, activated_all, eligible_count,
  fluency, response_hash }`. Never asserts before GATE; abstains
  (claim `None`) on unknown ground.
- `query_tuned(&stack, cue, QueryTuning)` — per-query knob overrides,
  clamped; `&self`, never mutates. `query_marking` is the one
  `&mut` query (stigmergy deposits only).
- `idle(&idle_stack, steps)` — time passes (decay), budgets hold.
- Traces carry `cue, evidence[], mass, support, ttl, hits,
  contradictions, tier (Core/Standard/Candidate), parents (imagine
  provenance), grain_members (renorm ledger), novelty (habituation),
  origin (causal attribution)`. `reliability() = 0.45·support +
  0.30·min(mass,1) + 0.25·(1−e^(−0.55·hits))`.
- Queries are READ-ONLY (Stage-9 law): 10 queries leave traces
  byte-identical (gated). Bonds echo, pheromone routes — evidence
  never moves at query time.
- Every mechanism ships **defaults-off and byte-identical when off**
  (D42 with/without law): each has config knobs, `QueryTuning`
  overrides where per-query makes sense, validation ranges, and gates
  proving both directions. Removing a law is an experiment with a
  record, not a reversion.

## Route the task

- Roots, laws, canonical cycles → read `docs/ROOTS.md`,
  `docs/ORDERING.md`, then [references/roots-and-ordering.md](references/roots-and-ordering.md).
- Engine API, traces, tiers, bonds, config → [references/engine-api.md](references/engine-api.md).
- Claim verifier, benchmark, free-text surface → [references/claim-verifier.md](references/claim-verifier.md).
- Retrieval primitives (W14–W18), KPR/EIG/imagine, the twelve
  admitted mechanisms → the **atom-memory-mechanisms** skill.
- Mother constitution, two-lane ask, reference pack, persona
  governor → the **atom-mother** skill.
- Admission protocol for anything new: typed signature, root
  expansion, defining invariant, matched baseline it must beat,
  held-out + counterfactual tests, ablation, adversarial failure
  test, zero-new-false-assertions check, proof it contributes
  something no existing primitive expresses
  (`docs/research/PRIMITIVE_CANDIDATES.md`).

## Non-negotiable invariants

1. Document-first: doctrine changes land in `docs/` (new D-entry in
   DECISIONS.md) before or with the code, never retrofitted.
2. No new mechanism without gates: matched baseline, ablation,
   adversarial test, determinism + validation + clamping, and the
   60-claim benchmark holding 60/60 with zero false assertions.
3. Defaults preserve validated behavior; toggles enable the
   with/without experiment. Never change a default silently.
4. `cargo test --workspace` plus `cargo fmt` / `clippy` clean before
   any claim of green. Known diagnostic: `tests/hops_probe.rs`
   (`hop1_under_hops_load`) is a **failing probe, not a gate** — a
   minimal reproduction of the multi-hop wall under heavy load
   (eligible_count 0 at hop 1 despite reliability ≈ 0.707); per D61
   the wall is compound (depth cap + 1/hop echo decay + routing
   margins), not a single law. Do not "fix" it by weakening a gate.
5. Zero third-party dependencies; no `unsafe`; pinned toolchain
   (`rust-toolchain.toml`).
