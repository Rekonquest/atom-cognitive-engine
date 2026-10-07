#!/bin/bash
# Sweep-export AtomDB passages from the read-only snapshot copy.
# Usage: ./export.sh  (run from rag-test/)
# Output: atomdb-corpus/packets.jsonl (raw retrieve packets, one per line)
set -u
CLI=./target-atomdb/release/atom-db
STORE=atomdb-copy/memory.atoms
OUT=atomdb-corpus/packets.jsonl
mkdir -p atomdb-corpus
: > "$OUT"
while IFS= read -r query; do
  [ -z "$query" ] && continue
  # shellcheck disable=SC2086
  "$CLI" retrieve "$STORE" $query 2>/dev/null | python3 -c "import json,sys; p=json.load(sys.stdin); [e.pop('threads',None) for e in p.get('evidence',[])]; print(json.dumps(p))" >> "$OUT"
done <<'QUERIES'
atom cognitive engine roots ordering
claim verifier benchmark false assertions
kinetic proofreading candidate promotion
expected information gain observation plans
renormalization pareto hedging stigmergy
precision bottleneck causal origins crdt merge
lateral inhibition quorum sensing
imagine blending creativity novelty
mother constitution echo bonds bridge
persona governor mode arousal restraint
reference pack child basics fever sleep
with without method defaults off gates
hysteresis canalization deferred controls watchlist
lightcone causal worlds geometry
atomdb memory store skills projects
atom writer humanizer project
quantum network keys sensors
mother ai core memories
rust workspace zero dependencies fail closed
cognitive engine query attention gate
conservation budget capacity eviction
decay reinforce revise contradiction
fluency contested sparse metacognition
robust aggregation adversary coalition
satisficing quorum deliberation effort
habituation novelty attention
QUERIES
echo "packets written to $OUT"
