---
name: atom-memory-mechanisms
description: Arm, tune, or audit the twelve admitted memory primitives and retrieval defenses of atom-cognitive-engine — KPR staging, EIG observation plans, imagine() blending, robust aggregation, fluency, Occam dedup, satisficing, habituation, renormalization, Pareto fronts, precision weighting, bottleneck compression, causal origins, CRDT merge, lateral inhibition, stigmergy, quorum sensing, and bet hedging. Use when the task concerns retrieval quality, adversarial robustness, memory dynamics, or the with/without arming table. Defaults-off discipline applies — never arm a mechanism without its measured guidance.
---

# atom-memory-mechanisms

Every mechanism here passed the same admission protocol: typed
signature, root expansion, defining invariant, matched baseline
beaten, held-out + counterfactual tests, ablation, adversarial
failure test, zero-new-false-assertions, and a contribution no
existing primitive expresses. Every one is **defaults-off and
byte-identical when off** (D42); every one ships knobs (config +
`QueryTuning` where per-query) with validation and clamping. New
work follows the same bar — see atom-cognitive-engine for the
protocol and the engine API.

## Retrieval stack order (load-bearing)

`eligible → dedup (W17) → lateral inhibition (#13) → robust cap/trim
(W14) → mixture → fluency read (W16) → satisficing gate (W18) /
quorum gate (#15) → assert`. Dedup runs BEFORE the cap (proven by
gate: correlated copies defeat cap-only 0.833 → stack wins 0.65).
Fluency is metadata (excluded from the response hash); Pareto is
metadata (never touches the mixture).

## Route the task

- Retrieval defense + metacognition + attention dynamics (W14, W16,
  W17, W18, W15) and the measured arming table →
  [references/retrieval-stack.md](references/retrieval-stack.md).
- The twelve admitted primitives (#1, #2, #4–#10, #13–#15),
  `imagine()`, the #16 rejection, deferred controls →
  [references/admitted-primitives.md](references/admitted-primitives.md).

## Arming guidance (measured D48/D49 — follow it, don't improvise)

- W14 ON everywhere (free defense; clean ground costs nothing).
- W18 ON where margins are wide (efficiency, answer-stable).
- W17 ONLY on cue-distinct surfaces (claim surface: safe no-op;
  Mother/cue-degenerate corpora: NO — flips 4/11 top behaviors).
- W15 ONLY where an observation loop runs (live traffic; dormant
  on static loads, unreachable per-query by design).
- Renormalization OFF on Mother as-encoded (re-ranking is
  acceptable-visible only where scale reduction is the goal).
- Precision (#7) + bottleneck (#8) pair intentionally: weighting
  casts small dissents, the bottleneck sweeps the irrelevant.
- Co-arming W14 + #13 measured SAFE (holds 0/20 poisoned).

## Invariants that hold across all mechanisms

1. Queries stay read-only (Stage-9): plain retrieval never writes
   traces, evidence, tiers, bonds, or pheromone — machine-checked
   (50 marking queries leave traces byte-identical). The testing-
   effect tension with this law is recorded as an operator decision,
   not a silent exception.
2. O3 admission ≠ license: expanded / bridged / merged members
   still clear the reliability floor; KPR candidates are
   query-invisible until promoted.
3. Convergence / compression / coarse-graining never manufacture
   confidence: rivals merge to CONTESTED, grains stay contested,
   fluency keeps reporting the true shape.
4. `cargo test --workspace` green + fmt/clippy clean; the 60-claim
   benchmark must hold 60/60, 0 false, with the mechanism armed.
