# Engine API

## Core types (`crates/atom-cognitive-engine/src/`)

- `config.rs` — `EngineConfig` (all knobs + fail-closed `validate()`).
  Key defaults: capacity 18, cue_dim 12, value_count 4, initial
  mass 1.0 / support 0.7, reinforce +0.18 mass / +0.12 support,
  contradiction factors 0.82 / 0.55, surprise_boost 1.5,
  decay 0.035 / 0.02, expire floors 0.10 / 0.55, match 0.72 /
  query 0.68, min_reliability 0.16, temperature 2.5, budget 12.0,
  ttl 8 / per-hit 2, bond depth 2 (max 8), echo bonuses 0.15 /
  0.15, kpr_min_stages 3, eig_margin 0.08 / eig_min_score 0.25,
  seed 2026_0821. All mechanism knobs default off (§ retrieval
  skill): robust cap 1.0 / trim 0, dedup 1.01, satisficing 0.0,
  habituation 0.0 / novelty 0.0, pareto false, hedge 0.0,
  stigmergy rate/gain 0.0, cap 4.0, decay 0.90, lateral 0.0,
  quorum 0, precision 0.0.
- `trace.rs` — `MemoryTrace`, `TraceTier`, `cosine_similarity`,
  `softmax`, `seeded_unit` (xorshift stream for deterministic
  tie-breaks + cooling).
- `engine.rs` — `CognitiveEngine`, `QueryTuning`, `QueryOutcome`,
  `ObserveOutcome`, `Transition`, reports (`FluencyReport`,
  `BlendReport`, `BottleneckReport`, `RenormalizationReport`,
  `MergeReport`, `QuorumReport`), `ObservationPlan`,
  `MarkingOutcome`, `EngineError`.
- `stacks.rs` — canonical + variant stacks (§ roots-and-ordering).
- `digest.rs` (core) — in-repo SHA-256 + hex; state/response hashes.

## Observe path

`observe(&observe_stack, cue, value, salience)` — validate →
predict (surprise boost on mismatch) → decay (time first) → attend
→ gate (match ≥ threshold: bind/reinforce; else nucleate if
capacity allows) → write → conserve (budget, weakest-first evict)
→ verify (hash). Contradiction on a live trace → REVISE:
dissolve-first-then-replace; Core tier counts but never dissolves;
Candidate tier resets stages. Variants: `observe_values` (multi-
value evidence — constitution ingestion), `observe_with_origin`
(causal attribution; 0 = unattributed, byte-identical default),
KPR `kpr_stack` (nucleations land Candidate; promotion after
`kpr_min_stages` distinct-tick contradiction-free stages; 3× faster
candidate decay; candidates query-invisible).

## Query path

`query(&stack, cue)` = `query_tuned` with config defaults.
validate → attend (temperature-scaled softmax over similarity ×
reliability + novelty term) → gate (query_threshold +
min_reliability eligibility) → expand (mother stack: BFS along
echo bonds to `bond_expansion_depth`, default 2, cap 8; expanded
members still clear the reliability floor) → retrieve (dedup →
lateral inhibition → robust cap/trim → mixture) → assert-or-abstain
(argmax value + confidence; `None` + empty = abstention) → verify.
`QueryOutcome` carries `claim`, `activations` (top-8 cited),
`activated_all` (full reachability set), `eligible_count`,
`fluency` (`FluencyReport { state, eligible_count, top_weight,
margin }` + `describe()`), `pareto_front` (when armed),
`quorum` report, `response_hash` (fluency and pareto excluded —
pure metadata, replay-identical).

`QueryTuning` (`from_config`, `clamped`) overrides: temperature,
query_threshold, min_reliability, bond depth, robust cap/trim,
dedup, satisficing, quorum, lateral. Floors: gate ≥ engine hard
floor; persona floors sit above (0.60 / 0.16).

## Bonds, tiers, merge, digest

- `add_bond(upstream, downstream)` — directional echo edges;
  consolidation flows upstream (`echo_support_bonus` on observe),
  expansion walks both directions depth-capped with
  `echo_activation_bonus`; queries never strengthen bonds
  (Stage-9). `clear_bonds` (placebo controls). D61: depth cap 8,
  default 2; depth-8 nine-fact chain crossing is the gate.
- `set_ingest_tier` — Core (identity substrate: no dissolve, never
  evicted, all-Core exhaustion fails closed) vs Standard (full
  lifecycle) vs Candidate (KPR probation).
- `merge(other)` (#9 CRDT): ancestor join (same id + same cue →
  G-Counter maxima, tier joins up, active ORs; same-id different-
  cue renumbers as fresh), fresh absorb with content-dedup
  idempotence, OR-set bonds (pheromone MAX, no self-loops).
  Commutative / associative / idempotent by canonical projection;
  convergence ≠ truth (rivals merge to CONTESTED).
- `state_digest()` — hash over traces AND bond pheromone (routing
  bias auditable without waiting for an observe); `transitions()`
  for the hash chain (O9).

## Generation and planning ops

- `imagine(cue, count)` (D38): top activated memories pair by rank
  (near-identical parents skipped); children = elementwise-mean
  cue/evidence recombinations born as KPR Candidates with
  `parents` provenance, weaker-than-committed mass/support. The
  ENVIRONMENT selects: recurring observations promote, dead
  regions dissipate. Novelty = 1 − max similarity to everything
  committed, parents included. Bounded count 1..=8.
- `recommend_observation(cue)` (D39): read-only, ≤3 ranked plans.
  Reinforce plans (reliability-blocked trace's own cue) and
  Explore plans (near-miss virgin territory inside `eig_margin`
  through KPR). Below `eig_min_score`: honest empty. Following a
  plan converts abstention → answer (the money gate). Scores are
  deterministic gap proxies, never probabilities.
- `renormalize(granularity)` (#5), `compress_evidence(dust)` (#8) —
  see atom-memory-mechanisms.
