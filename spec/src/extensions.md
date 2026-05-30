# Extensions

OMIR's core deliberately captures only the ~80% of fields that ~80% of memory systems
need (see [Design Principles §3](./principles.md#3-8020-core-plus-extensions)). The
remaining long tail — vendor-specific scores, experimental signals, model-specific
metadata — is carried in the typed `extension[]` array. This is OMIR's escape hatch:
it lets implementations round-trip their proprietary data through a standard format
**without** breaking core conformance and **without** every other consumer needing to
understand it.

## The extension mechanism

Every R1 resource (`MemoryRecord`, `Entity`, `Relationship`, `Episode`) carries an
optional `extension[]` array. Each element is an **Extension** object:

| Field | Type | Required | Meaning |
|---|---|---|---|
| `url` | URI | **yes** | Canonical URL that *defines* this extension. |
| `valueString` | string | no | Scalar string value. |
| `valueNumber` | number | no | Scalar numeric value. |
| `valueBoolean` | boolean | no | Scalar boolean value. |
| `valueJson` | any JSON | no | Arbitrary structured value. |

Rules:

- An Extension **MUST** carry a `url`. The `url` is the extension's identity: it tells
  a consumer *what this is* and where the definition lives.
- An Extension **SHOULD** carry exactly one `value*` field. Use `valueJson` when the
  payload is structured (an object or array); use the scalar `value*` fields for
  simple values.
- A consumer **MUST** ignore any extension whose `url` it does not recognize, and
  **MUST** still process the resource. Unknown extensions are **never** an error. This
  is the rule that makes the format forward-compatible: a producer can add new
  extensions and old consumers keep working.
- Extensions **MUST NOT** be used to override or contradict a core field. They *add*
  data; they do not *replace* it.

## The extension URL registry

An extension `url` is a stable, dereferenceable identifier for an extension
definition. OMIR adopts a registry model (as FHIR does):

- **Standard extensions** defined by the OMIR Working Group live under the OMIR
  namespace, e.g. `https://omir.io/spec/R1/extension/<name>`.
- **Vendor extensions** live under a URL the **vendor controls**, e.g.
  `https://veld.dev/omir/ext/<name>`. A vendor **MUST NOT** mint extensions under the
  `omir.io` namespace.
- The URL **SHOULD** resolve to a human- and machine-readable definition (name,
  description, value type, and the resources it applies to).
- The Working Group maintains a public index of standard extension URLs. Vendor
  extensions **MAY** be listed there for discoverability but are not owned by the WG.

This keeps the core small and stable while letting the ecosystem innovate at the edges:
two implementations can each carry rich proprietary data, exchange Bundles, and lose
nothing — each ignores the other's extensions and reads the shared core.

## Worked example: Veld's 20-signal retrieval scores

Veld's retrieval pipeline computes a 20-signal score vector per memory (recency,
arousal, source credibility, graph strength, cross-encoder blend, entity match, and so
on). None of that belongs in the OMIR core — it is a Veld implementation detail. It
rides in an extension instead.

```json
{
  "resourceType": "MemoryRecord",
  "id": "m-042",
  "content": "Switched the cross-encoder to track HF main and pre-warm on startup.",
  "createdAt": "2026-05-30T16:10:00Z",
  "kind": "learning",
  "importance": 0.74,
  "confidence": { "alpha": 7, "beta": 2, "calibrated": 0.78 },
  "extension": [
    {
      "url": "https://veld.dev/omir/ext/scoring-signals",
      "valueJson": {
        "schema": "veld-20-signal",
        "version": "0.7.6",
        "signals": {
          "recency": 0.91,
          "arousal": 0.40,
          "sourceCredibility": 0.85,
          "graphStrength": 0.62,
          "crossEncoder": 0.71,
          "entityMatch": 0.55,
          "tagMatch": 0.33,
          "episodeCoherence": 0.48,
          "activationLevel": 0.66
        }
      }
    },
    {
      "url": "https://veld.dev/omir/ext/external-dimensions",
      "valueJson": {
        "density": 0.58,
        "coherence": 0.72,
        "closure": 0.41,
        "confidence": 0.79,
        "isotropy": 0.63
      }
    }
  ]
}
```

What this buys each party:

- **Veld** round-trips its full retrieval state through `.omir` and reconstructs it on
  import — nothing is lost in export.
- **A generic consumer** (another agent, a viewer, a different memory engine) ignores
  both extension URLs it does not recognize and processes the record using only the
  core fields (`content`, `importance`, `confidence`, …). It still works.
- **A second Veld-aware tool** recognizes `https://veld.dev/omir/ext/scoring-signals`,
  pulls the signal vector out of `valueJson`, and uses it.

This is the 80/20 rule in action: the shared 80% is interoperable by everyone; the
proprietary 20% travels safely alongside it without becoming everyone's problem.
