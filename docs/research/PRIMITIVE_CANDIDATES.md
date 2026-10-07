# Primitive candidates — the v0.7 program

Status: opened 2026-08-23. Source: the operator's candidate list (16
cross-domain primitives) plus the creativity first-principles framework.
Admission protocol (operator-specified, non-negotiable): **none of these
enters by name alone.** Each must arrive with (1) a typed signature, (2) a
proposed expansion over the eight roots, (3) its defining invariant, (4) a
matched existing-composition baseline it must beat, (5) held-out and
counterfactual tests, (6) an ablation, (7) an adversarial failure test,
(8) a zero-new-false-assertions check, and (9) a demonstration that it
contributes something no existing primitive expresses.

Stacking order matters (operator law): serial before parallel, gating
before blending, single-scale before multiscale. The order below follows
the operator's recommendation.

## The queue

**Reordered 2026-08-23 per D40 (operator directive: discovery before
controls).** The control family — hysteresis (#3), canalization (#11),
integral adaptation (#12) — is DEFERRED until the instability signs
appear (chattering, mode thrash, route flapping, uncorrected drift; see
D40's watchlist). Capability and connection primitives come first.

| Priority | # | Candidate | Domain | One-line mechanism | Key risk |
|---|---|---|---|---|---|
| ✅ built | 1 | **Kinetic proofreading** | immunology | evidence must survive staged verification before commitment; failure dissipates | latency; rejects valid-but-sparse evidence |
| ✅ built | 2 | Expected information gain | experimental science | unknown → the observation/intervention that most reduces uncertainty | wrong uncertainty model picks useless experiments |
| ✅ built | 5 | Multiscale renormalization | stat. physics | coarse-grain while tracking what was removed | removes variables that later matter |
| ✅ built | 6 | Pareto preservation | multi-objective opt | keep all non-dominated trade-offs, don't force one score | front grows unboundedly |
| ✅ built | 10 | Bet hedging | evolution | keep bounded-diverse hypotheses, not just the leader | spends resources on current losers |
| ✅ built | 14 | Stigmergy | collective systems | worlds leave bounded work-traces for other worlds | self-reinforcing fake evidence — traces never count as truth |
| ❌ mapped | 16 | Catalytic routing / exaptation | chemistry / evolution | influence pathways without becoming evidence; recruit old structure for new use | hidden routing bias — must appear in traces |
| ✅ built | 7 | Precision-weighted prediction error | cognitive science | update strength = error × estimated source precision | self-reinforcing confidence |
| ✅ built | 8 | Information bottleneck | information theory | compress retaining target-predictive information | relevance is target-dependent |
| ✅ built | 4 | Causal event ordering | distributed computing | happened-before vs concurrent, never arrival-order-as-causal | metadata + sync complexity |
| ✅ built | 9 | CRDT convergence | distributed systems | order-independent deterministic merges | convergence ≠ truth |
| ✅ built | 13 | Lateral inhibition | developmental bio | neighbors inhibit → specialization | forces artificial differences |
| ✅ built | 15 | Quorum sensing | microbiology | expensive deliberation only on distributed support | correlated worlds fake a quorum |
| deferred | 3 | Hysteresis | control/biology | separate enter/exit thresholds prevent chattering | preserves wrong laws too long — needs an emergency exit |
| deferred | 11 | Canalization | developmental bio | stabilize trajectories against noise, stay sensitive to true contradiction | hides fragility |
| deferred | 12 | Integral adaptation | chemotaxis | accumulate persistent error to correct systematic bias | windup overshoot |

## Creativity first-principles → root mapping

The operator's creativity framework, mapped honestly onto what the engine
already expresses (and what it lacks):

| Creativity principle | Engine expression today | Status |
|---|---|---|
| Blind variation + selective retention | selection = GATE/promotion; variation = candidates pool (via KPR, #1) | **KPR completes the pair** — and is the D9 re-earning path for the demoted EXPLORE: candidates ARE the bounded variations, promotion IS the retention |
| Conceptual blending / bisociation | echo bonds compose facet-dissimilar memories; cross-domain bridges | partially expressed |
| Constraint-based creativity | hard constraints = gate floors, tier protection; soft = persona governor | expressed |
| Novelty + value | novelty = surprise (PREDICT's error); value = reliability/support | novelty measured, value not optimized |
| Problem reframing | absent — queries arrive pre-formed | gap |
| Tension as driver | conflict → abstention, not synthesis | gap (Pareto #6 partially addresses) |
| Memory + recombination | traces + bonds | expressed |

Honest note: creativity principles 5 and 6 are GAPS, not strengths. The
roadmap should not claim them until built.

## Progress log

- **#1 kinetic proofreading — BUILT as v0.7** (D35): see DECISIONS and
  tests/kpr.rs. Root expansion: DECAY (fast candidate dissipation) + BIND
  (staged accumulation) + GATE (promotion threshold on independent
  stages) + REVISE (contradiction resets accumulation — the KPR reset) +
  CONSERVE (bounded candidate pool). Admission discipline met: matched
  baseline (record-stack single-stage ingestion), ablation (promote stage
  omitted → nothing ever commits), adversarial reset test, zero new false
  assertions (candidates are query-invisible), and the contribution proof
  (single-touch noise that asserts under the baseline abstains under KPR).

- **Creativity layer #1 — `imagine()` (D38, v0.8)**: conceptual blending
  built as an engine operation. Top activated memories pair by rank;
  children are cue/evidence recombinations born as KPR Candidates with
  parent provenance; the ENVIRONMENT selects (recurring evidence →
  promotion; dead regions → dissipation). Novelty = 1 − max similarity
  to everything committed (parents included). Six gates in
  tests/creativity.rs. Completes blind variation + selective retention;
  novelty and value are now both measurable terms.

- **#2 expected information gain — `recommend_observation()` (D39,
  v0.9)**: unknown → a productive next step. Reinforce plans target
  reliability-blocked traces (observe its cue to unlock); Explore plans
  probe near-miss virgin territory through KPR; below the score floor
  the honest answer is "no available observation reduces this
  uncertainty." Seven gates in tests/eig.rs — the money gate: FOLLOWING
  the plan converts an abstaining query into an answering one, both
  kinds. Scores are deterministic gap proxies, never probabilities.

## Wave 2 — the operator's open question (2026-08-24): "what else adds to intelligence?"

The first list was strong on verification, optimization, and coordination.
Wave 2 opens the untouched territories: TIME, ANALOGY, COUNTERFACTUALS,
CATEGORIES, SELF-KNOWLEDGE, INSIGHT. Sized small to large per the
operator ("pieces could be small and big, some obvious some not
obvious"). Same admission protocol; D40's discovery-first ordering
applies. Sources are named for verifiability; entries are candidates

### The with/without audit (D42): "did I add rules too soon?"

Laws classified by evidence basis — the honest answer to the operator's
question, and the ablation-experiment queue:

| Class | Laws | Verdict |
|---|---|---|
| EARNED (measured failure receipts) | forget-before-learn, order-before-attention, gate-before-blend, preload-before-traffic, acyclic compositions | keep — each was paid for by a recorded 0/4 or interference result |
| ADMISSION-GATED (this engine) | KPR staging, tiers, bond depth, expansion reliability floor | keep — each has a failing-ablation proof |
| INHERITED TASTE (no local receipt) | read-only-queries (Stage-9), O10 weaken-harsher, two-lane separation, the 0.75 verifier threshold (thin margin) | ABLATION CANDIDATES — each gets a with/without experiment before it is considered permanent truth |

The first inherited-taste experiment is already flagged by science: the
testing effect (retrieval strengthens memory — Roediger & Karpicke)
contradicts read-only-queries directly. Operator's call when run.
(hypotheses), not verified fits, until each passes its gates.

### Large — new reasoning classes

| # | Candidate | Source | Mechanism → engine meaning | Risk |
|---|---|---|---|---|
| W1 | **Analogy / structure mapping** | Gentner; Hofstadter | match RELATIONAL structure, not surface similarity → blend across the echo graph by role/structure, not cue distance; the general form of the kitchen→surveillance bridge | surface matches masquerading as deep ones |
| W2 | **Counterfactual simulation** | Pearl rung 3 | answer "what if X had differed" by re-running composition with X altered — treated/baseline exists in the causal world; the ENGINE lacks it | simulated worlds mistaken for observed ones — must carry a simulated flag that bars evidence use |
| W3 | **Active intervention** | Pearl rung 2; experimental method | EIG recommends observations; intervention EXECUTES a chosen probe and learns from the outcome — with an explicit permission gate (operator law: external actions need consent) | acting to confirm one's own hypothesis |
| W4 | **Theory of mind** | Premack & Woodruff; Tomasello | agent-separated belief tracking: the protectee's state modeled as a SEPARATE trace space from self-state — never merged (two-lane law applies per agent) | projecting self-model onto others |
| W5 | **Self-model** | metacognition literature | an expected-performance model of the engine itself: which regions it knows well, where it habitually abstains, its own drift — a map of blind spots built from its own query history | self-description ossifying into self-limitation |
| W6 | **Insight operations** | Knoblich, Ohlsson et al. | constraint relaxation + chunk decomposition — the two operations that unlock insight problems; directly fills the PROBLEM REFRAMING gap (D36) | relaxing the wrong constraint confidently |

### Medium — new structure classes

| # | Candidate | Source | Mechanism → engine meaning | Risk |
|---|---|---|---|---|
| W7 | **Episodic segmentation** | Zacks et al., event segmentation theory | detect event boundaries (prediction-error spikes) → close one memory episode, open the next; gives memory temporal SHAPE instead of an undifferentiated stream | boundary artifacts fragmenting continuity |
| W8 | **Offline replay** | hippocampal replay (Wilson & McNaughton lineage) | reactivate and recombine EXISTING traces without new input — imagine() running on stored material during idle; consolidation by rehearsal in absence of stimulus | replayed content gaining unearned confidence — replay must strengthen BONDS, not evidence |
| W9 | **Prototype formation** | Rosch typicality | extract the typical member of a trace cluster as its own trace (fast classification via prototype, exemplars retained for edge cases) | prototype drift as category drift; typical ≠ true |
| W10 | **Multi-metric attention** | multi-head attention (transformers) | several similarity spaces in parallel (cue, cast, echo-graph distance, provenance) attending independently and voting — one softmax today | head count explosion; votes needing the duplicate-world guard |
| W11 | **Successor representation** | Dayan 1993 | predict FUTURE STATES, not just values → prospective traces: "what usually follows what" enables planning by forward simulation | mistaking succession for causation |
| W12 | **Valence-weighted consolidation** | emotion-enhanced memory (McGaugh) | memories tagged good/bad consolidate at different rates — high-|valence| traces decay slower; ties directly to salience and the wound-engine | trauma-style over-persistence; needs contradiction as the emergency exit |
| W13 | **Explanation-as-test** | self-explanation effect (Chi) | knowledge verified by GENERATING an explanation of it — a trace that cannot support an explanation of its own evidence is demoted | fluent rationalization passing as understanding |

### Small — unglamorous, high-leverage

| # | Candidate | Source | Mechanism → engine meaning | Risk |
|---|---|---|---|---|
| W14 | **Robust aggregation** | robust statistics | median/trimmed aggregation alongside means → one poisoned trace cannot drag a mixture; small code, big effect on noisy worlds | discarding genuine outliers that were the signal |
| W15 | **Habituation / sensitization** | Groves & Thompson 1970 dual process | repeated stimulus → response decrement (habituation); strong stimulus → increment (sensitization) → attention stops saturating on constants; protects against ruts | habituating to the important-but-constant |
| W16 | **Retrieval fluency as metacognitive signal** | tip-of-the-tongue research | the SHAPE of retrieval (score spread, latency proxy, eligible count) is itself evidence about confidence — not just the answer | fluency ≠ correctness; calibration required |
| W17 | **MDL / Occam selection** | Solomonoff; minimum description length | when two laws/traces explain the same evidence, keep the compressively shorter — ties to the ≥3× compression gates' philosophy, made a selection law | short-but-wrong beats long-but-right |
| W18 | **Satisficing** | Simon bounded rationality | know when to STOP thinking: an effort-accuracy floor that returns the current best answer instead of maximal deliberation | premature termination on hard problems |

### Already half-present (recorded, not re-queued)

- **Spacing effect** (Ebbinghaus's flip side): KPR's distinct-tick rule
  already encodes distributed-beats-massed — the engine obeys spacing
  without having named it. Worth a gate asserting it explicitly.
- **Generation effect** (self-generated content consolidates stronger):
  a TESTABLE PREDICTION of the existing system — do imagine()-born
  blends promote on fewer observations than passively observed ones? If
  yes, the effect emerged unforced. Experiment, not a build.

### One genuine tension surfaced (operator's call)

The **testing effect** (retrieval practice strengthens memory more than
re-observation — Roediger & Karpicke) conflicts with the house law
"queries are read-only; reinforcement is explicit action" (Stage-9,
atom-db). Humans strengthen by retrieving; this engine deliberately
does not. Options: keep the law (purity, costs the effect) / add an
explicit-action retrieval mode (query-with-intent-to-learn) / leave
deferred. This changes an authority boundary, so it is the operator's
decision, not mine.

## Progress log — the five-primitive run (2026-08-25, D49–D53)

- **#5 multiscale renormalization — BUILT (D49)**: `renormalize(granularity)`
  merges ≥-granularity-similar traces into grains; evidence conserved
  exactly, fine ledger kept, bonds rewired, contradiction SPLITS the grain
  (rollback). Corpus-scale discovery: NOT answer-invariant under
  attention-weighted aggregation (7/13 Mother top-behaviors re-ranked;
  family held; fluency stayed contested — no manufactured confidence);
  3.5× working-set reduction measured. OFF on Mother as-encoded (same
  verdict as W17, now with conservation + rollback).
- **#6 Pareto preservation — BUILT (D50)**: queries armed with
  `pareto_front` return the non-dominated contender set over
  (reliability, support, novelty). Names tension instead of collapsing
  it; pure metadata (claim/activations/hash byte-identical on/off);
  bounded; brute-force-verified non-domination.
- **#10 bet hedging — BUILT (D51)**: the attention tail (lowest-standing
  fifth of committed traces) decays at (1−hedge_rate) and keeps ttl.
  Measured: regime-flip retention (unhedged abstains, hedged answers with
  ~3× accumulated evidence); falsified traces still expire (hedge delays,
  never cancels — 6 vs 70 steps, dead in both).
- **#14 stigmergy — BUILT (D52)**: bounded pheromone on bonds; marking
  queries (explicit action) deposit on walked edges; evaporation on the
  time base; routing reads trails, evidence never does (machine-checked:
  50 marking queries leave traces byte-identical); pheromone rides in the
  state hash + new public `state_digest()`. Coordination payoff measured:
  marked routes strengthen far-member retrieval; effects fade with
  evaporation and saturate at the cap.
- **#16 catalytic routing / exaptation — REJECTED as new mechanism,
  MAPPED with receipts (D53)**: its capabilities are {stigmergy
  (pathways-not-evidence), bonds+imagine (recruitment + provenance)};
  the draft hub-filter cannot distinguish hub from bridged answer without
  an oracle. Receipts in tests/exaptation_mapping.rs.

- **#7 precision-weighted prediction error — BUILT (D54)**: revision
  strength = error (the existing surprise boost) × source precision
  (salience relative to standing reliability). Weak pings nudge, strong
  contradictions keep force (no lock-in), consistent weak signals still
  accumulate (bounded skepticism — measured flip at ~10-12 pings vs
  control's 2-3). Claim benchmark holds armed. Root expansion: PREDICT
  (error term) + REVISE (weighted dissolve/replace) + reliability as
  the precision estimator.

## Progress log — the final five (2026-08-25, D55–D59)

- **#8 information bottleneck — BUILT (D55)**: `compress_evidence(dust)`
  prunes sub-share evidence dust; argmax provably retained (dust ≤
  1/(2·values)); removed mass accounted exactly; pairs with #7 (precision
  weighting casts small dissents, the bottleneck sweeps the irrelevant).
- **#4 causal event ordering — BUILT (D56)**: `observe_with_origin`;
  same-origin revisions supersede, two explicit origins are CONCURRENT —
  no dissolve, both masses live, arrival-order permutation invariance
  machine-checked (shares equal to 1e-6).
- **#9 CRDT convergence — BUILT (D57)**: `engine.merge(other)` —
  ancestor join (G-Counter maxima), fresh absorb with content-dedup
  idempotence, OR-set bonds; commutative/associative/idempotent up to
  canonical content; convergence ≠ truth: rivals merge to CONTESTED
  ground. The engine is composable (worlds unite).
- **#13 lateral inhibition — BUILT (D58)**: the winner softly suppresses
  near-neighbors (×(1−g·sim²)), renormalized toward distinct voices;
  winner never changes; W14 co-arm measured SAFE on the battery.
- **#15 quorum sensing — BUILT (D59)**: expansion gated on DISTINCT
  supporters (correlated copies count once); QuorumReport says why
  deliberation ran; singleton fast path keeps the answer.

**Queue status: ALL SIXTEEN PROCESSED.** Built: #1, #2, #4, #5, #6, #7,
#8, #9, #10, #13, #14, #15 (twelve mechanisms). Rejected-as-mapped: #16.
Deferred per D40 (operator law, awaiting instability signs): #3
hysteresis, #11 canalization, #12 integral adaptation.
