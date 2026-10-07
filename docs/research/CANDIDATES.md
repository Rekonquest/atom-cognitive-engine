# Candidate cognitive first principles

Status: surveyed 2026-08-21. Every candidate below is assessed in `ROOTS.md`
against the admission filter. Nothing here is admitted by default; presence in
this file means "considered," not "true."

Sources are grouped: (A) human-cognition literature, (B) theoretical frames,
(C) receipts from the Atom ecosystems (atom-harness, atom-db, the memory
program) — cited as `repo:path:line` from the atom-harness clone at
`~/Desktop/Projects/atom-harness`.

## A. Human cognition candidates

| Candidate | Core claim | Notes for the filter |
|---|---|---|
| Attention | Selection: relevance-weighted narrowing from all to some | Implemented as a recipe everywhere in the ecosystem; softmax weighting + thresholds |
| Association / binding | Links form between co-active cue and content | Hebbian in spirit; the bind/nucleate fork is the load-bearing decision point |
| Reinforcement | Use/repetition strengthens a link | Measured asymmetric laws exist in the receipts (strengthen gentler than weaken) |
| Decay / forgetting | Unused or unsupported state weakens toward neutral | Ebbinghaus curves; ttl/support/mass decay in receipts; also capacity-forgetting |
| Consolidation | Labile state stabilizes into durable structure (offline replay) | Crystallization gates in receipts require repetition + support thresholds |
| Prediction-error learning | Learning is driven by the gap between predicted and observed, not by exposure alone | Rescorla–Wagner; dopamine prediction-error; **user-mandated inclusion** |
| Predictive processing | The system constantly predicts its input; errors propagate and drive update | Precision-weighting is attention-like; see free-energy frame below |
| Abstraction | Repeated related instances compress into shared structure | Compression gates in receipts (ratio ≥ 3×); "abstract = remember across repetition" |
| Inhibition / gating | Signals are passed, blocked, or routed by context compatibility | Context-gating and guarded-relay receipts exist because ungated transfer failed |
| Working-memory limits | Buffers are bounded; overflow forces eviction | Capacity=18 retirement in receipts; conservation budgets |
| Motivation / homeostasis | Setpoints and drives regulate the whole system | The homeostatic governor exists in the ecosystem as a complete lane |
| Pattern separation vs completion | Similar inputs kept distinct vs merged for recall | The match-threshold fork implements exactly this pair |
| Interference | Prior learning corrupts new learning (and vice versa) | The memory program's "outcome 3"; a failure mode to be *governed*, not a principle |
| Serial order | Sequence structure is itself information | "Order discarded before attention" is a recorded 0/4 failure |
| Curiosity / information gain | Uncertainty itself drives exploration | Stage-8 information-gain activation exists in receipts |

## B. Theoretical frames

- **Predictive processing / free-energy principle / active inference.** The
  user's stated intuitions — prediction as part of cognition, entropy,
  emergence — converge on this family: a system survives by minimizing
  surprise (long-run entropy of its observations); it predicts, acts to make
  its predictions true, and updates when surprised. Relevance here: it gives
  PREDICT a first-class seat and explains EXPLORE (expected information gain)
  and CONSERVE (resistance to dispersion) in one frame.
- **Chaos / complexity.** Bounded exploration during formation (annealing,
  chaos budgets), monotonic cooling, edge-of-order dynamics. Receipts: chaos
  budget 1.25, monotonic-cooling gates, behavior-preserving chaos gates.
- **Emergence.** Weak emergence (compositional novelty) is the working
  assumption: nothing is admitted that requires strong emergence; the stack's
  compositions must be expandable and acyclic (receipt: cyclic compositions
  rejected fail-closed).

## C. Receipts from the Atom ecosystems (the "hard way" record)

1. **Forget-before-learn.** `revise = (forget, remember)` —
   `repo:atom_runtime_knowledge.py:160`; "Dissolve contradicted structure
   **before** learning replacement evidence" — `repo:atom_runtime_knowledge.py:132-136`;
   `remember = (forget, learn), "apply time, then reshape"` —
   `repo:atom_universe_composition.py:200-204`; embodied at
   `repo:atom_cognitive_experiment.py:373` (decay precedes bind) and `:396`
   (conserve follows the write).
2. **Order must exist before attention.** Averaged summaries destroyed symbol
   order and scored a measured 0/4 —
   `repo:docs/systems/ATOM_GENERATIVE_ENGLISH_DEV_NOTES.md:70-71`.
3. **Gate/route before blend; evidence constrains, never overrides.** Same
   doc `:74-75`; lane-arbiter admission does not license an answer —
   `repo:docs/notes/DEVELOPER_NOTES.md:148-153`.
4. **Preload before traffic.** Capabilities load before any question data is
   routed — `repo:atom_provider_fabric.py:357`, `repo:docs/notes/DEVELOPER_NOTES.md:648-650`.
5. **Import/order fragility is real.** A recorded break fixed by reordering —
   `repo:docs/notes/DEVELOPER_NOTES.md:2171-2174`.
6. **Acyclic composition enforced fail-closed.** "Cyclic Atom composition" —
   `repo:atom_universe_composition.py:237-248`.
7. **Reordered state is rejected.** Resume contract rejects altered/skipped/
   duplicated/reordered state — `repo:README.md:311-315`.
8. **Asymmetric learning law.** Wrong predictions cost more than right
   predictions gain (−350 vs +200 per mille) — atom-db causal-memory
   conductance; −400 vs +250 in stage 9.
9. **Interference is the default cross-domain failure.** Memory program
   2026-06-11: domain corpora interfered ("outcome 3"); routing only the
   active domain's memories mitigated it.
10. **Open divergence to settle here.** `learn` = attention→nucleation→
    conservation at `repo:atom_universe_composition.py:191-199`, but
    nucleation→attention→conservation at `repo:atom_runtime_knowledge.py:156`.
    The executable evidence (`repo:atom_cognitive_experiment.py:384` before
    `:393`) follows attention-first.

## D. Candidates explicitly rejected as roots (recorded in DECISIONS.md)

Populated by `ROOTS.md`; each rejection names which root composes it.
