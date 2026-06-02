# Relationship

> **Generated artifact.** This page is generated from [`schemas/Relationship.schema.json`](../../../schemas/Relationship.schema.json) and [`schemas/common.schema.json`](../../../schemas/common.schema.json). Do not hand-edit; regenerate from the schema and the field tables will stay authoritative.

A directed, weighted edge between two `Entity` resources. Strength is dynamic (Hebbian): it increases with co-activation and decays without use.

**Maturity:** OMM-3

## Purpose

A `Relationship` is a typed graph edge linking two entities. Its `strength` carries Hebbian synaptic weight, and `validAt` / `invalidatedAt` give it a temporal lifecycle so superseded edges can be retained rather than deleted.

> See [Memory Semantics §4](../semantics.md#4-hebbian-relationship-strength) for how `strength` rises with co-activation and decays with disuse.

## Fields

| Field | Type | Card. | Description |
|---|---|---|---|
| `resourceType` | `"Relationship"` (const) | 1..1 | Discriminator. MUST be `Relationship`. |
| `id` | `Id` (string) | 1..1 | Resource-local identifier, unique among `Relationship` resources within a Bundle. Pattern `^[A-Za-z0-9._:-]{1,128}$`. |
| `from` | `Reference` | 1..1 | Source `Entity` reference, e.g. `{ "ref": "Entity/john" }`. |
| `to` | `Reference` | 1..1 | Target `Entity` reference. |
| `relationType` | string (open vocabulary) | 1..1 | Edge type. Common values below. Implementations MAY mint new lowercase `snake_case` values. |
| `meta` | `Meta` | 0..1 | Metadata envelope (`omirVersion`, `profile[]`, `source`, `createdAt`, `lastUpdated`, `maturity`). |
| `strength` | `UnitInterval` ([0,1]) | 0..1 | Synaptic weight. Dynamic under Hebbian plasticity. |
| `context` | string | 0..1 | Free-text note describing the relationship. |
| `createdAt` | `Instant` (date-time) | 0..1 | When the edge was first recorded. |
| `validAt` | `Instant` (date-time) | 0..1 | When this relationship was last observed to hold (temporal tracking). |
| `invalidatedAt` | `Instant` (date-time) | 0..1 | When this relationship was invalidated, if ever (temporal edge invalidation). |
| `sourceEpisode` | `Reference` | 0..1 | `Episode` reference that produced this relationship. |
| `extension` | array of `Extension` | 0..* | Typed, namespaced extensions carrying implementation-specific data without breaking core conformance. |

`additionalProperties` is `false`: a conformant `Relationship` carries only the fields above.

> **Reference constraint.** A `Reference.ref` matches `^(MemoryRecord|Entity|Relationship|Episode)/…`. By convention `from` and `to` point to `Entity` resources and `sourceEpisode` points to an `Episode`; the schema's `Reference` shape does not itself narrow the target type, so producers MUST honor these conventions.

### `relationType` common values

`works_with`, `works_at`, `employed_by`, `part_of`, `contains`, `owned_by`, `located_in`, `located_at`, `related_to`. The vocabulary is open.

## Minimal

The required set is `resourceType`, `id`, `from`, `to`, `relationType`.

```json
{
  "resourceType": "Relationship",
  "id": "rel-varun-omir",
  "from": { "ref": "Entity/varun" },
  "to": { "ref": "Entity/omir" },
  "relationType": "maintains"
}
```

## Full

```json
{
  "resourceType": "Relationship",
  "id": "rel-varun-omir",
  "meta": {
    "omirVersion": "R1",
    "source": "veld/0.7.6",
    "maturity": 3
  },
  "from": { "ref": "Entity/varun" },
  "to": { "ref": "Entity/omir" },
  "relationType": "maintains",
  "strength": 0.88,
  "context": "Convenes the OMIR working group.",
  "createdAt": "2026-05-30T10:05:00Z",
  "validAt": "2026-05-30T11:42:00Z",
  "invalidatedAt": null,
  "sourceEpisode": { "ref": "Episode/ep-launch-chat" },
  "extension": [
    {
      "url": "https://veld.dev/omir/ext/hebbian",
      "valueJson": { "coActivations": 17, "lastStrengthened": "2026-05-30T11:42:00Z" }
    }
  ]
}
```

> The `null` for `invalidatedAt` above is illustrative of an edge still in force; producers SHOULD simply omit the field when an edge has not been invalidated.

## References

A `Relationship` points to other resources by typed `Reference` (`ResourceType/id`):

- **`from` → `Entity`** — the source endpoint of the edge.
- **`to` → `Entity`** — the target endpoint of the edge.
- **`sourceEpisode` → `Episode`** — the episode that produced this relationship.

No `Relationship` field references `MemoryRecord` or `Bundle`.
