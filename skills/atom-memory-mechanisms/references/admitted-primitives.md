# Admitted primitives

Status: sixteen-queue fully processed (D59, 174 tests green) —
built #1, #2, #4–#10, #13–#15; #16 rejected-as-mapped; #3, #11,
#12 deferred per D40. Queue lives in
`docs/research/PRIMITIVE_CANDIDATES.md`; Wave-2 hypotheses
(W1–W18: analogy, counterfactuals, intervention, theory of mind,
self-model, insight, episodic segmentation, offline replay,
prototypes, multi-metric attention, successor representation,
valence consolidation, explanation-as-test…) are UNBUILT
hypotheses until gated.

## #1 Kinetic proofreading (D35, immunology)

Third tier `Candidate`: staged, uncommitted, query-invisible.
`observe.gate_candidate` lands nucleations there (relaxed bind
threshold); `observe.promote` advances distinct-tick
contradiction-free stages; ≥ `kpr_min_stages` (default 3, 1..=16)
promotes; contradiction RESETS to zero. 3× faster dissipation.
Roots: DECAY + BIND + GATE + REVISE (reset) + CONSERVE. Gates
(`tests/kpr.rs`, 7): dissipation, promotion, reset, matched
baseline (one-touch noise ASSERTS record-baseline, ABSTAINS KPR —
the contribution proof), promote-ablation (nothing commits),
10-noise storm → zero assertions, distinct-tick bar. Completes
BLIND VARIATION + SELECTIVE RETENTION (D36) — the D9 re-earning
path for demoted EXPLORE. Canonical observe + record ingestion
unchanged.

## `imagine()` creativity layer (D38)

Conceptual blending as engine op: top activated memories pair by
rank (near-identical skipped); children = elementwise-mean
cue/evidence recombinations, born KPR Candidates with `parents`
provenance (hash-visible), weaker initial mass/support.
Environment selects (recurring evidence → promote; dead regions →
dissipate). Novelty = 1 − max similarity to ALL committed,
parents included (excluding them scored orthogonal and
near-parent blends identically — caught by gate: 0.293 vs
0.184). Six gates (`tests/creativity.rs`): candidates-not-
assertions, both-parent inheritance, novelty measured,
ENVIRONMENT SELECTS, parents never mutated, bounded 1..=8 +
empty/single-trace no-ops. Lessons: near cues BIND existing
traces (compression by design); "parents unchanged" brackets the
op itself (later observes decay everything — engine time).
Value = reliability/support at promotion. Roots: BIND + ATTEND +
KPR's GATE/DECAY.

## #2 Expected information gain (D39)

`recommend_observation(cue)`: unknown → productive next step.
Reinforce plans (reliability-blocked trace's own cue —
following it unlocks via normal reinforcement) and Explore plans
(near-miss virgin territory inside `eig_margin` 0.08 via KPR).
Below `eig_min_score` 0.25: honest empty ("no available
observation reduces this uncertainty"). Read-only, ≤3 ranked,
deterministic gap proxies — NOT probabilities. Money gate:
FOLLOWING the plan converts abstention → answer, both kinds.
Never mutates (byte-identical); never itself an assertion. Setup:
fully reinforced ≈ 0.97 reliability — block with low-salience
commits; default gate is 0.68 (Mother config 0.60 — compute
near-miss windows against the right threshold). Gates
(`tests/eig.rs`, 7). Roots: PREDICT + ATTEND + GATE + CONSERVE.

## #5 Multiscale renormalization (D49)

`renormalize(granularity)`: ≥-granularity cue-similar active
pairs merge into GRAINS (seed = lowest id); evidence sums
exactly (minority dissent survives inside), support/mass keep
max, hits/contradictions add, novelty max. Fine ledger
(`grain_members`), bonds rewire to grains, contradiction SPLITS
(grain dies with post-merge earnings; members re-activate with
frozen evidence; rewiring NOT undone — recorded asymmetry).
Candidates never merge. Explicit only; 1.01 = off, byte-identical.
Gates (`tests/renormalization.rs`, 5): exact conservation,
majority preserved (anti-W17 gate), ~25% dissent survives, split,
rewire + reachability. Corpus discovery: 121 → 34 active (3.5×
reduction, all evidence retained) BUT 7/13 top behaviors re-rank
(protectee probe P-001 → P-005, family held) — coarse-graining
is NOT answer-invariant under attention-weighted aggregation;
fluency stayed contested (no manufactured confidence). OFF on
Mother as-encoded.

## #6 Pareto preservation (D50)

`pareto_front` queries return the NON-DOMINATED FRONT over
(reliability, support, novelty) — contenders no other trace
dominates (≥ all axes, > one). Names tension (habituated champion
+ fresh contender both stand; gate: size 2, values 1 and 2).
Pure metadata: claim/activations/hash byte-identical on/off
(gated); 60/60 armed. Bounded by eligible count; deterministic
(reliability desc, id asc); O(n²) exact; brute-force verified.
Default off (empty front).

## #10 Bet hedging (D51, evolution)

`hedge_rate` 0..=0.5 (default 0): attention tail (lowest fifth of
committed, reliability-ranked, id ties) decays at (1−rate), keeps
ttl — losers persist as regime-flip insurance. Measured: long
quiet → unhedged ABSTAINS, hedged answers with ~3× accumulated
evidence. Delays, never cancels: falsified traces still expire
(~6 vs ~70 idle steps, dead both). Candidates never hedge; hard
floors absolute. 60/60 armed. Gates (`tests/bethedge.rs`, 3).
Fixture: contradictions must run while the victim still binds
(revise refreshes ttl); single-member weights read 1.0 — measure
retention in evidence, not claim weight.

## #14 Stigmergy (D52, collective systems)

Bonds carry `pheromone` (0.0 default). `query_marking` (explicit
`&mut` action) deposits `stigmergy_rate` on walked edges, capped
at `pheromone_cap` 4.0; `pheromone_decay` 0.90 evaporates per
decay tick; retrieval reads `stigmergy_gain × pheromone` atop the
echo bonus. Routing-not-evidence law, machine-checked: 50 marking
queries leave every TRACE byte-identical. Stage-9 holds (plain
queries never deposit; `query_tuned` is `&self`). Visibility law:
pheromone rides the state hash + public `state_digest()`. Payoff
measured: marked routes weigh far (hop-2) members higher; fades
with evaporation; saturates at cap. Zero stays zero — unarmed
byte-identical. Gates (`tests/stigmergy.rs`, 5).

## #16 rejected-as-mapped (D53)

Fails the novelty bar with receipts (`tests/exaptation_mapping.rs`,
3): pathways-not-evidence IS stigmergy (re-proven in recruitment
geometry, traces byte-identical); recruit-old-structure IS bonds +
D38 provenance (bridged member JOINS with weight ≥ 0.10, cited,
never outvotes the asking facet's anchor — attention math
T·(sim + ln rel): sim-0 bridge can't beat sim-1 anchor).
Recruitment supplements, never overrides. Draft hub-filter
rejected: can't distinguish hub from bridged answer without an
oracle. Hidden-bias risk barred: bridged members ride in the
activation set AND response hash.

## #7 Precision-weighted prediction error (D54)

`precision_gain` 0..=1 (default 0): revise scales by source
precision — w = strength/(strength + gain·reliability),
strength = effective_salience/2; support cost, evidence dissolve
(factor^w), dissent add all scale by w. Error term pre-existed
(surprise boost); #7 adds the precision term. Measured: 0.25
ping vs 10-hit belief → armed retains >1.5× evidence + small
nonzero dissent (heard, not obeyed); 3 full-salience (2.0)
contradictions flip even 12-hit maximal belief (escape hatch —
no lock-in); consistent 0.4 pings flip moderate belief ~10–12
vs control ~2–3 (bounded skepticism). 60/60 armed (correction
at salience 1.0 still revises). O10 stands at unit precision.
Gates (`tests/precision.rs`, 4). Roots: PREDICT + REVISE.

## #8 Information bottleneck (D55)

`compress_evidence(dust)`: prune evidence entries below
`dust` × trace total. Argmax provably retained (validation
enforces dust ≤ 1/(2·value_count)). Removed mass accounted
EXACTLY (1e-12 gate). Pairs with #7 (weighting casts small
dissents, bottleneck sweeps the irrelevant). Answers keep value
AND fluency shape (the bottleneck constraint). dust 0 = no-op.
60/60 armed. Relevance is share-relative by design (same mass
survives poor, dies rich). Gates (`tests/bottleneck.rs`).

## #4 Causal event ordering (D56)

`observe_with_origin` (0 = unattributed = byte-identical
default, hash-proven). Same-origin revisions supersede (corpus
correction untouched). Two DIFFERENT explicit origins are
CONCURRENT: later arrival never dissolves standing belief (no
silent last-writer-wins); dissent joins at precision weight (#7
composes); BOTH masses stay live (standing keeps ≥30% after 8
concurrent vs 5). Money gate: arrival-order permutation
converges to mirrored distributions (1e-6); the QUERY follows
mass, not order. Cross-origin confirmations reinforce +
re-attribute. Origin rides the state line. Lesson: a silently
no-op'd fixture patch was caught by the concurrency gate's 0.55²
residue — gates doing their job. Gates (`tests/causal.rs`, 4).

## #9 CRDT convergence (D57)

`engine.merge(other)` — worlds unite deterministically (join-
semilattice, three passes: ancestor join / fresh absorb /
bonds — see engine-api). Commutative / associative / idempotent
by canonical projection (ids are engine-local ordinals).
Convergence ≠ truth (gated risk): rivals merge to CONTESTED.
Fail-closed on dim mismatch. Engine is Clone (replica forking).
Gates (`tests/crdt.rs`, 5).

## #13 Lateral inhibition (D58)

After dedup, before the cap: winner suppresses near-neighbors —
weight ×(1 − g·sim²) vs the top member's cue (quadratic:
distinct voices intact, redundant chorus fades); mixture
re-normalizes toward distinct contributors. 0..=1, default 0.
Gates (`tests/lateral.rs`, 4): distinct share rises ≥1.5×;
mutually-distinct untouched to 1e-9; same-value chorus never
changes the WINNER. Interaction measured: co-armed W14 + .8
HOLDS 0/20 (cap does the lifting; coalition shrinks equally) —
safe on this geometry. Fixture: distinct member must not
already be reliability winner.

## #15 Quorum sensing (D59, microbiology)

`quorum_min` 0..=64 (default 0 = always deliberate): expansion
runs only on DISTRIBUTED support — eligible set clusters at cue
sim 0.90, correlated copies count ONCE (four identical copies
read ONE supporter — the risk column, gated by construction).
`QuorumReport { distinct_supporters, quorum_met }` says WHY
deliberation ran. Below bar: fast path, anchor still answers
(savings without answer cost). Composes with W18; the D40
watchlist's ready brake. Gates (`tests/quorum.rs`, 4).

## D60 lightcone-core port (program open, staged gates)

Operator-authorized port of the harness reasoning taxonomy +
creativity engine: D60a typed relations on bonds (RelationKind:
supports/enables/clarifies/grounds/qualifies/corroborates/
defeats/contradicts/undercuts/counterexample; signed warrant;
hash-bound overlay; untyped default bit-identical); D60b the 44
reasoning-branch schemas as DATA (configure D60a over
composition paths; shuffled-schema control); D60c CreativityEngine
module (7 primitives, 8 stress families ≥ harness thresholds,
violations fail closed). Correction (primary source): the
ORIGINAL lightcone is geometric, hashless — the typed graph is
the harness implementation layer, not the concept; destination
is the UNION (typed warrant enforcing geometry holding trained
worlds). Planner in scope (route-select-strategize:
multi_world_deliberation / decompositional_causal_chain /
fast_causal_recall / active_world_search). Nothing hardcoded
until D60a-c gates pass.
