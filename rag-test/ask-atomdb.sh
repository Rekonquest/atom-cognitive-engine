#!/bin/bash
# ask-atomdb.sh "question" — AtomDB recall + cognitive-engine judgment.
# 1. atom-db CLI retrieves a shortlist from the snapshot copy (read-only).
# 2. Shortlist passages become engine traces (ids kept as citations).
# 3. The engine queries the shortlist: assert/cite or abstain honestly.
# Usage: ./ask-atomdb.sh "what are the eight cognitive roots"
set -u
cd "$(dirname "$0")"
Q="$*"
[ -z "$Q" ] && { echo "usage: ./ask-atomdb.sh \"question\""; exit 2; }
# shellcheck disable=SC2086
./target-atomdb/release/atom-db retrieve atomdb-copy/memory.atoms $Q > /tmp/atomdb-shortlist.json 2>/dev/null
python3 - "$Q" <<'EOF'
import json, sys
seen = {}
with open('/tmp/atomdb-shortlist.json') as f:
    raw = f.read()
try:
    pkt = json.loads(raw)
except Exception:
    pkt = {}
for ev in pkt.get('evidence', []):
    pid = ev.get('identity')
    if not pid or pid in seen:
        continue
    seen[pid] = {'id': pid, 'source': (ev.get('sources') or ['unknown'])[0], 'text': ev.get('text', '')}
with open('/tmp/atomdb-shortlist.jsonl', 'w') as f:
    for p in seen.values():
        f.write(json.dumps(p) + '\n')
print(f"shortlist passages: {len(seen)}", file=sys.stderr)
EOF
cargo run -q -p simple-rag -- /tmp/atomdb-shortlist.jsonl "$Q"
