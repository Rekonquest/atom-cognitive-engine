# Persona governor (D33–D34)

Mother as ACTIVE component (operator: "put the persona in the
cognitive engine so she has more control"). `PersonaGovernor`
(`cli/src/persona.rs`) tracks MODE + AROUSAL and prescribes a
`QueryTuning` for the next query. Authority boundary: HOW she
attends, never WHAT is true.

## State

- MODE: majority dominance among top-5 cited activations —
  civilian (identity_dominant) / operational
  (operational_dominant) / integrated. Data-grounded: protectee
  scenarios read INTEGRATED (every approved bucket-9 protectee
  memory is integrated dominance — the a-priori "operational"
  expectation was wrong about the corpus, recorded honestly).
- AROUSAL: half-life decay (`arousal * 0.5 + pressure`,
  clamped 0..1); rises with a pressure signal per cycle.

## Prescription (`decision(&base)` → `GovernorDecision { mode,
arousal, tuning, citations }`)

- Operational + P-002: temperature +0.5 (cap 6.0), gate +0.05.
- Civilian + C-001: temperature +0.5, gate −0.03 (floored).
- Integrated + R-005: baseline maintained.
- Arousal > 0.6 + SG-004/SG-001: gate +0.10, reliability +0.10,
  echo depth −1 — assert less, abstain more (restraint as engine
  behavior). Every delta carries a behavior-code citation.
- Absolute floors: gate ≥ 0.60, reliability ≥ 0.16 (above the
  engine hard floors 0.5/0.05); `clamped()` before return. Even
  a corrupted decision cannot dismantle the gate.

## Gates (`tests/persona_suite.rs`)

Mode tracking (data-grounded INTEGRATED), restraint-under-
pressure (governed eligible set ⊆ ungoverned; no new assertions;
SG-004+SG-001 cited), no-mutation authority (trace/bond state
byte-identical after governed queries), absolute floors.
Engine support: `query_tuned` per-query overrides; `query()` is
defaults.

## Demo

```bash
cargo run -p atom-cognitive-cli -- --persona-demo
```

Full loop live: baseline knobs → calm protectee moment (mode +
arousal + citations) → 3 pressurized cycles (tightened knobs) →
corrupted-ground query governed-vs-plain (subset admission,
fewer assertions) → authority statement. Battle-testing
deferred per operator.
