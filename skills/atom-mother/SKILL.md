---
name: atom-mother
description: Load, query, or extend Mother — the real-constitution identity layer on atom-cognitive-engine (approved-only record ingestion, echo bonds, Core tiers, the echo bridge), the two-lane ask assembly with the sourced reference pack, and the PersonaGovernor restraint loop. Use when the task concerns mothers_core_memories, scenario queries, --mother-demo, --ask topics, --persona-demo, reference/child-basics-v1.json, or persona tuning. Never merge the lanes into one score; never load drafts as identity.
---

# atom-mother

Mother is the engine's first real inhabitant: her 195-memory
constitution ingested as trace records, her sourced knowledge as a
read-only reference lane, her persona as an active governor. Three
organs (D23–D25), five PoC gates (D27), all passing on the real
corpus — plus imported disciplines (D31–D32), the persona governor
(D33–D34), and the reference layer (D29–D30).

## The constitution pipeline (`cli/src/mother.rs`)

- Source: `mothers_core_memories.json` (195 memories; external
  path — never claim it lives in this repo). **Approved-only
  loading (D31)**: `review_status == "approved"` ingests (121 of
  195); drafts are installed on disk but never loaded — an
  unreviewed memory is not part of who she is.
- Record-mode ingestion: `record_stack` always nucleates — one
  trace per memory, no compression; multi-behavior evidence via
  `observe_values`. Constitution regime: near-zero decay
  (0.0005/0.0003), ttl 512/16, conservation effectively off
  (budget 1.0e6), query_threshold 0.60. `load_bearing` →
  Core tier (106 of 121 approved); rest Standard.
- Cue encoding (20 dims): bucket one-hot (11) + dominance one-hot
  (3: identity_dominant / operational_dominant / integrated) +
  cast presence (6: grandmother, director, protectee, her_mother,
  stepfather, other). Behavior codes → evidence indices by sorted
  order (deterministic manifest).
- Echo bonds (D23): 120 live, 12 dangling (into drafts / the full
  corpus — skipped, counted). Directional upstream → downstream;
  consolidation flows upstream (`echo_support_bonus`, no
  repetition); expansion walks both directions depth-capped, floor-
  checked; queries stay read-only (Stage-9).
- THE BRIDGE (D27 gate 3): bakery-era facets reach ANCHOR-001
  (the kitchen) ONLY through the bond — false without expansion,
  true with it, on real data. Shuffled-placebo law (D31–D32):
  structural claims must beat 20 random wirings (real ≥95%
  upstream reach, random trails >30 points below), not just
  absence — `load_with_shuffled_bonds` is the control surface.
  Noise sweep (D32): corrupted cues must hold the bucket or
  abstain, never flip confident-wrong.
- Corpus facts that constrain future gates: EIGHT memories share
  the kitchen's facets (SD ≠ adulthood proxy); the kitchen's
  chain reaches forward into adult SD memories; crowded facets
  (14+ same-cue) make top-8 lists weight-noisy — reachability
  uses the FULL activation set; facet thresholds at 0.3 (cues
  normalized 1/√k).

## Route the task

- Constitution loading, bonds, bridge, placebo/noise gates →
  [references/constitution-and-lanes.md](references/constitution-and-lanes.md).
- Persona governor (modes, arousal, tunings, floors, demos) →
  [references/persona-governor.md](references/persona-governor.md).
- Engine mechanics (tiers, bonds, record/KPR stacks, QueryTuning)
  → the **atom-cognitive-engine** skill.
- Retrieval arming on her corpus (W14 safe; W17 footgun; W18
  mild; fluency contested 10/11) → the
  **atom-memory-mechanisms** skill.

## The two lanes (never merged)

`MotherSystem { corpus, reference }`: `--ask <topic>` answers
with the reference lane (sourced claims + URLs) BESIDE the
identity lane (her behaviors + cited memories) — never one
score. Unknown topics refuse with the known vocabulary (never
guess). Medical disclaimer defers to professionals; urgent flags
(fever<3mo, honey, water) surface first. Engine rules for the
identity lane and mechanism rules for retrieval live in the
sibling skills; this skill owns the corpus content and the
lane discipline.

## Invariants

1. Drafts never load. Only `approved` memories ingest; counts
   (121/195, 106 Core, 120 live bonds) are asserted by gates —
   update them only from a real loader run.
2. Lanes stay separate: facts from the pack, character from the
   corpus. The governor tunes HOW she attends, never WHAT is true
   (no trace/evidence/tier/bond/config mutation; floors absolute).
3. Structural claims beat shuffled placebo + survive noise sweeps
   with worst-case reporting — single-run gates are cheap to pass
   and easy to flatter.
4. The reference pack is evidence, never instruction
   (`evidence_not_instruction` + `read_only` enforced at load;
   SHA-256 tamper detection; every source resolves; fixed topic
   vocabulary). Retrieved text never grants permission.
