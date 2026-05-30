# Bundle

> **Generated artifact.** This page is generated from [`schemas/Bundle.schema.json`](../../../schemas/Bundle.schema.json) and [`schemas/common.schema.json`](../../../schemas/common.schema.json). Do not hand-edit; regenerate from the schema and the field tables will stay authoritative.

A serialized collection of OMIR resources — the on-disk `.omir` document itself.

**Maturity:** n/a — `Bundle` is the container/transport envelope, not a graded memory resource.

## Purpose

The `Bundle` is the container resource: a `.omir` file *is* a `Bundle`. It carries a flat `entry[]` of core resources whose typed references (`ResourceType/id`) resolve within the bundle, and it is JSON-LD compatible — an optional `@context` enables linked-data processing without changing the core shape.

## Fields

| Field | Type | Card. | Description |
|---|---|---|---|
| `resourceType` | `"Bundle"` (const) | 1..1 | Discriminator. MUST be `Bundle`. |
| `omirVersion` | `"R1"` (const) | 1..1 | Spec release the bundle conforms to. MUST be `R1`. |
| `entry` | array of resource (`oneOf` MemoryRecord \| Entity \| Relationship \| Episode) | 1..* | The resources carried by this bundle. Order is not significant; references resolve by `ResourceType/id`. |
| `@context` | string or object (JSON-LD context) | 0..1 | Optional JSON-LD context. SHOULD be `https://omir.io/spec/R1/context.jsonld` or an object that includes it. |
| `id` | `Id` (string) | 0..1 | Resource-local identifier for the bundle. Pattern `^[A-Za-z0-9._:-]{1,128}$`. |
| `generatedAt` | `Instant` (date-time) | 0..1 | When the bundle was serialized. |
| `source` | string | 0..1 | Producing implementation, e.g. `"veld/0.7.6 (MIF adapter)"`. |

`additionalProperties` is `false`: a conformant `Bundle` carries only the fields above. The schema marks `required: ["resourceType", "omirVersion", "entry"]`; `entry` is an array, so a valid bundle contains at least one resource.

> **File profiles.** The canonical encoding is JSON / JSON-LD with extension `.omir`. The compact binary profile (CBOR/bincode) uses extension `.omirb` and carries the identical resource model.

## Minimal

The required set is `resourceType`, `omirVersion`, `entry`.

```json
{
  "resourceType": "Bundle",
  "omirVersion": "R1",
  "entry": [
    {
      "resourceType": "MemoryRecord",
      "id": "mem-001",
      "content": "OMIR is the at-rest format MCP transports.",
      "createdAt": "2026-05-30T11:42:05Z"
    }
  ]
}
```

## Full

```json
{
  "@context": "https://omir.io/spec/R1/context.jsonld",
  "resourceType": "Bundle",
  "omirVersion": "R1",
  "id": "demo-bundle-001",
  "generatedAt": "2026-05-30T12:00:00Z",
  "source": "veld/0.7.6 (MIF adapter)",
  "entry": [
    {
      "resourceType": "Episode",
      "id": "ep-launch-chat",
      "content": "Varun and the agent agreed OMIR should be positioned as the at-rest format MCP transports, not a competitor to it.",
      "source": "message",
      "eventTime": "2026-05-30T11:42:00Z",
      "createdAt": "2026-05-30T11:42:03Z",
      "entityRefs": [
        { "ref": "Entity/varun" },
        { "ref": "Entity/omir" }
      ]
    },
    {
      "resourceType": "Entity",
      "id": "varun",
      "name": "Varun",
      "labels": ["person"],
      "salience": 0.91,
      "properNoun": true
    },
    {
      "resourceType": "Entity",
      "id": "omir",
      "name": "OMIR",
      "labels": ["project", "concept"],
      "salience": 0.97,
      "properNoun": true
    },
    {
      "resourceType": "Relationship",
      "id": "rel-varun-omir",
      "from": { "ref": "Entity/varun" },
      "to": { "ref": "Entity/omir" },
      "relationType": "maintains",
      "strength": 0.88,
      "sourceEpisode": { "ref": "Episode/ep-launch-chat" }
    },
    {
      "resourceType": "MemoryRecord",
      "id": "mem-positioning",
      "content": "Position OMIR as the at-rest data format that MCP/A2A transport, not a competing protocol.",
      "kind": "learning",
      "experienceType": "decision",
      "tier": "longterm",
      "createdAt": "2026-05-30T11:42:05Z",
      "importance": 0.95,
      "confidence": { "alpha": 9.0, "beta": 1.0, "calibrated": 0.9 },
      "entityRefs": [
        { "ref": "Entity/omir" },
        { "ref": "Entity/varun" }
      ]
    }
  ]
}
```

## References

The `Bundle` does not hold typed `Reference` fields of its own; it is the resolution scope for everyone else's. Within a single bundle:

- **`entry[]`** may contain **`MemoryRecord`**, **`Entity`**, **`Relationship`**, and **`Episode`** resources.
- Cross-resource references (`MemoryRecord.entityRefs`, `Episode.entityRefs`, `Relationship.from` / `to` / `sourceEpisode`) resolve against the `entry[]` members by their `ResourceType/id`.

A `Bundle` MUST NOT appear inside another `Bundle`'s `entry[]` — the `oneOf` admits only the four core resources.
