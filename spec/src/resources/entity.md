# Entity

> **Generated artifact.** This page is generated from [`schemas/Entity.schema.json`](../../../schemas/Entity.schema.json) and [`schemas/common.schema.json`](../../../schemas/common.schema.json). Do not hand-edit; regenerate from the schema and the field tables will stay authoritative.

A named thing extracted from memory — a person, place, organization, concept, technology, product, skill, or discriminative keyword.

**Maturity:** OMM-3

## Purpose

An `Entity` is a canonicalized node in the knowledge graph. It is the target of `MemoryRecord.entityRefs` and `Episode.entityRefs`, and the endpoint type for every `Relationship`. Salience governs how strongly an entity pulls on retrieval.

## Fields

| Field | Type | Card. | Description |
|---|---|---|---|
| `resourceType` | `"Entity"` (const) | 1..1 | Discriminator. MUST be `Entity`. |
| `id` | `Id` (string) | 1..1 | Resource-local identifier, unique among `Entity` resources within a Bundle. Pattern `^[A-Za-z0-9._:-]{1,128}$`. |
| `name` | string | 1..1 | Canonical surface form, e.g. `"John"`, `"Paris"`, `"Rust programming"`. |
| `meta` | `Meta` | 0..1 | Metadata envelope (`omirVersion`, `profile[]`, `source`, `createdAt`, `lastUpdated`, `maturity`). |
| `labels` | array of enum (12 values, see below) | 0..* | One or more type labels. Open at the edges via `other` + `extension`. |
| `summary` | string | 0..1 | Context summary built from surrounding relationships. |
| `mentionCount` | integer ≥ 0 | 0..1 | Number of times this entity has been mentioned. |
| `salience` | `UnitInterval` ([0,1]) | 0..1 | How important this entity is. Higher salience = stronger gravitational pull in retrieval. |
| `properNoun` | boolean | 0..1 | Proper nouns carry higher base salience than common nouns. |
| `attributes` | object (string → string) | 0..1 | Type-specific key/value attributes. |
| `createdAt` | `Instant` (date-time) | 0..1 | When the entity was first recorded. |
| `lastSeenAt` | `Instant` (date-time) | 0..1 | When the entity was most recently observed. |
| `extension` | array of `Extension` | 0..* | Implementation-specific data (e.g. name embeddings — model + vector — ride here; embeddings are NOT core in R1). |

`additionalProperties` is `false`: a conformant `Entity` carries only the fields above.

### `labels` vocabulary

`person`, `organization`, `location`, `technology`, `concept`, `event`, `date`, `product`, `skill`, `keyword`, `project`, `other`.

## Minimal

The required set is `resourceType`, `id`, `name`.

```json
{
  "resourceType": "Entity",
  "id": "varun",
  "name": "Varun"
}
```

## Full

```json
{
  "resourceType": "Entity",
  "id": "omir",
  "meta": {
    "omirVersion": "R1",
    "source": "veld/0.7.6",
    "maturity": 3
  },
  "name": "OMIR",
  "labels": ["project", "concept"],
  "summary": "Open Memory Interoperability Resources — a FHIR-style standard for portable agent memory.",
  "mentionCount": 34,
  "salience": 0.97,
  "properNoun": true,
  "attributes": { "kind": "standard", "release": "R1" },
  "createdAt": "2026-05-30T10:00:00Z",
  "lastSeenAt": "2026-05-30T11:42:00Z",
  "extension": [
    {
      "url": "https://veld.dev/omir/ext/name-embedding",
      "valueJson": { "model": "minilm-384", "dim": 384 }
    }
  ]
}
```

## References

An `Entity` holds **no outbound references** to other OMIR resources — it is a leaf node by design. The graph edges around it are expressed elsewhere:

- **`Relationship.from` / `Relationship.to` → `Entity`** — entities are the endpoints of every relationship.
- **`MemoryRecord.entityRefs[]` → `Entity`** and **`Episode.entityRefs[]` → `Entity`** — records and episodes point *to* entities.
