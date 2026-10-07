# Retrieval stack: W14, W16, W17, W18, W15

The small-piece set (D43–D47): five defaults-off pieces, one stack
story — robust aggregation defends the mixture, dedup collapses
correlated voters, fluency reads the shape, satisficing spends
effort accordingly, habituation keeps attention on what changes.

## W14 robust aggregation (D43, `robustify_weights`)

TRIM (`robust_trim` lowest-weighted members zeroed) + CAP (no
member exceeds `robust_weight_cap` of total; excess redistributed
water-filling). Breakdown bound: one confident-but-wrong trace
cannot outvote a coalition; a genuine majority still wins.
Defaults cap 1.0 / trim 0; per-query via QueryTuning (clamped
0.15..=1.0 / 0..=8). Gates (`tests/robust.rs` + CLI): adversary
0.672 → capped 0.35 → coalition 0.65 → correct; trim alone does
NOT stop a heavy adversary (cap is load-bearing); determinism,
validation, clamping; 60/60 benchmark under robust ON (cap 0.5 +
trim 1). Motivation receipt: the causal lane's 28-wrong-confident
/ two-above-0.90 false-assertion finding.

## W16 fluency (D44, `FluencyReport`)

The SHAPE of retrieval as metacognitive signal (tip-of-the-tongue
lineage). Checked in order: Blank (nothing eligible = the
abstention) → Contested (top two values within 0.15) → Sparse
(≤2 supporters) → Fluent (concentrated, clear margin). Fields
`{ state, eligible_count, top_weight, margin }` + `describe()`.
Honesty condition: wrong-but-unopposed consensus reads FLUENT —
the engine can feel certain and be wrong, exactly like a person;
fluency is metadata (hash-excluded) and W14 exists as the separate
defense. Blind-spot discovery (kept): under PLAIN aggregation a
confident adversary reads Fluent (0.67 share, wide margin); only
robust capping exposes the true margin. Gates: `tests/fluency.rs`.

## W17 Occam/dedup (D45, `dedup_correlated`)

Weight-rank visit; any member with CUE similarity ≥
`dedup_threshold` to a kept representative has its weight DROPPED
(copies don't vote twice). Survivors renormalize; then W14 caps.
Defaults 1.01 (off); validated 0.5..=1.01; per-query. Closes
W14's hole: three correlated adversary copies defeat cap-only
(wrong holds 0.833) — dedup collapses them, coalition wins 0.65.
Distinct traces (≈0.61) survive (W16 cluster intact — gate). Dedup
alone returns the strongest copy's value — only the STACK wins.
Discovery (kept): KPR-committed near-copies MERGE at observe time;
only RECORD-mode documents carry the duplicate-vote hazard into
retrieval — how it arrives in real corpora. **Footgun (D48)**:
cue-degenerate corpora (Mother's identical-cue facets) collapse
whole facets — 4/11 top behaviors flip. Arm ONLY cue-distinct.
Gates: `tests/occam.rs`.

## W18 satisficing (D46)

Simon's stopping rule as a conditional in `step_query_expand`:
when `satisficing_margin` > 0 and the eligible-only margin meets
the bar, bond expansion SKIPS (fast path, effort saved measured);
below the bar, full deliberation. Default 0.0 (always deliberate);
validated 0..=1; per-query. Composition W16→W18 paying off:
fluency instruments the margin, satisficing consumes it. Gates
(`tests/satisficing.rs`): fast path earns its skip (identical
claim, strictly fewer members); contested ground still fully
deliberates (activation sets identical). Setup notes: contested
needs DIFFERENT values both sides (same-value reads Sparse);
claim tuples carry float confidence — compare VALUE with epsilon.

## W15 habituation/sensitization (D47, Groves & Thompson)

`novelty` per trace (1.0 = novel), dynamics entirely OBSERVE-side:
confirmation decays (`novelty *= 1 − habituation_rate`);
contradiction/surprise restores 1.0. Retrieval only READS
(`+ novelty_gain × novelty` in the logit; 0 = term vanishes) —
Stage-9 survives by design. Defaults 0/0 (validated 0..=0.5 /
0..=1); Core habituates too; blends born 1.0. Gates
(`tests/habituation.rs`): fresh outranks habituated at equal
similarity (with/without control falls back to reliability
ranking); surprise re-sensitizes; 10 queries byte-identical.
Honest measurement (D48): ALONE it leaves the adversary
undefeated (20/20) — allocation policy, not defense; dormant on
fresh static loads (uniform novelty cancels in softmax).

## Realtime sweep (D48, `tests/realtime_sweep.rs`)

- Clean 60-claim: every arm 60/60, 0 false, 12 abstain — DO-NO-HARM.
- Adversarial battery (20 poisoned + 20 clean): unarmed 20/20
  wrong → W14 0/20; W14+W17, +W18 also 0/20; zero clean regressions.
- Mother real corpus (11 scenarios): W14 stable 11/11; W18
  answer-stable, 25.9 → 24.8 mean activation; W17 flips 4/11
  (footgun); fluency reads 10/11 CONTESTED, 1 blank, 0 fluent —
  her facets genuinely disagree and the engine says so.
