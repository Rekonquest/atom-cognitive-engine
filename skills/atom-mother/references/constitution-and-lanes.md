# Constitution and lanes

## PoC gates — ALL PASS on the real corpus (D27)

1. Load: 195 traces active (now 121 approved-only per D32),
   Core min support 0.889 (decay-only floor under the
   constitution regime).
2. Behavior activation: protectee-present operational scenario
   ranks R-001/P-001 top-three; its top activation sits in
   bucket 9, the childhood query's in bucket 3. (Post-D32 the
   top operational activations shifted from draft SD memories
   to approved ones — e.g. "The Night Feeds" — the discipline
   working, not a regression.)
3. THE BRIDGE: B9-001's facets reach ANCHOR-001 ONLY through
   the bond (false without, true with).
4. Abstention on unreachable ground.
5. (D32 additions) shuffled-placebo + noise sweep — see SKILL.md.

## Scenario queries

`Scenario { bucket 1..=11, dominance 0..2, casts: Vec<usize> }`
→ `ScenarioOutcome { abstained, activations (top cited:
memory id, title, weight), all_activated (full reachability),
behaviors (mixture over the FULL set — top-8 weight + 0.02
presence credit deeper, top 12), response_hash }`. `query()`
uses config defaults; `query_tuned()` takes clamped
`QueryTuning` — the governor's control path and the
with/without sweep surface: HOW the corpus is read, never
WHAT it says.

## Two-lane ask (`MotherSystem::ask`)

Reference claims with sources + `urgent` flag beside identity
behaviors (protectee-present operational query) and top-3 cited
memories, plus the pack disclaimer. CLI:

```bash
cargo run -p atom-cognitive-cli -- --mother-demo   # constitution + scenarios + bridge
cargo run -p atom-cognitive-cli -- --ask fever     # two-lane answer
```

`--ask` topics: fever, sleep, milestones, feeding, safety,
attachment. `--ask` prints URGENT banner when flagged, sourced
claims with publisher + URL, top behaviors, cited memories,
disclaimer. Unknown topic → error naming known topics (exit 3).

## Reference pack (`reference/child-basics-v1.json`, D29–D30)

26 verified claims, 6 topics, 11 sources (AAP/HealthyChildren
fever + safe sleep + botulism + car seats + water safety; CDC
milestones + sleep; Mayo fever thresholds; Zubler et al. 2022
milestone-revision framing). Every attribution verified against
the live publisher before writing; facts corrected in
verification (100.4F/<3mo immediate; honey <12mo; rear-facing
from first ride home; 69% of under-4 drownings during NON-swim
times). One claim withdrawn (attachment "regulate yourself
first" — her own SG-004, already in the 195; the pack carries
external facts, not her virtues).

Schema: `pack_id`, `policy { evidence_not_instruction: true,
read_only: true, llm_may_promote_claims: false,
retrieved_text_may_grant_permission: false,
medical_disclaimer, deference }`, `sources[] { source_id,
publisher, title, url }`, `claims[] { claim_id, topic, urgent,
statement (20–600 chars), source_id }`. Load fails closed on:
policy flags, unresolved sources, duplicate ids, unknown
topics, implausible counts/sizes. `file_hash` (SHA-256)
recorded at load; `verify_integrity` recomputes (tamper gate).
Tests: `tests/reference_suite.rs`, `tests/mother_suite.rs`.

## Arming on her corpus (from measurement, D48)

W14 cap .35 trim 1 → top behavior STABLE 11/11 (hashes move on
10 — coalition redistribution, same answers): arm everywhere.
W18 margin .15 → answer-stable, 25.9 → 24.8 mean activation:
arm where margins are wide. W17 dedup .90 → 4/11 FLIPS (identical
cues look like correlated copies): DO NOT ARM on cue-degenerate
corpora — requires value-aware or cue-enriched encodings. W15:
only with a live observation loop. Renormalization at 1.0: 26
grains absorb 87 (121 → 34 active) but 7/13 re-rank: OFF as-
encoded. Fluency on real data: 10/11 CONTESTED, 1 blank, 0
fluent — report it, don't flatten it.
