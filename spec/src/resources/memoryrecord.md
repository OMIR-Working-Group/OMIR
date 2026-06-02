# MemoryRecord

> **Generated artifact.** This page is generated from [`schemas/MemoryRecord.schema.json`](../../../schemas/MemoryRecord.schema.json) and [`schemas/common.schema.json`](../../../schemas/common.schema.json). Do not hand-edit; regenerate from the schema and the field tables will stay authoritative.

The atomic unit of agent memory: a remembered experience, plan, prompt, or learning.

**Maturity:** OMM-4

## Purpose

A `MemoryRecord` is the smallest retrievable item of cognitive state — one remembered thing, with the context needed to score, decay, and re-surface it. One core record type carries several lifecycle classes (`memory`, `plan`, `prompt`, `learning`) so that retrieval stays unified across them.

> See [Memory Semantics](../semantics.md) for how `confidence`, `decay`, `tier`, and `provenance` behave over time.

## Fields

| Field | Type | Card. | Description |
|---|---|---|---|
| `resourceType` | `"MemoryRecord"` (const) | 1..1 | Discriminator. MUST be `MemoryRecord`. |
| `id` | `Id` (string) | 1..1 | Resource-local identifier, unique among `MemoryRecord` resources within a Bundle. Pattern `^[A-Za-z0-9._:-]{1,128}$`. |
| `content` | string | 1..1 | WHAT — the primary human-readable content of the memory. |
| `createdAt` | `Instant` (date-time) | 1..1 | Encoding time: when the record was written. |
| `meta` | `Meta` | 0..1 | Metadata envelope (`omirVersion`, `profile[]`, `source`, `createdAt`, `lastUpdated`, `maturity`). |
| `kind` | enum `memory` \| `plan` \| `prompt` \| `learning` | 0..1 | Record lifecycle class. Default `memory`. |
| `experienceType` | enum (14 values, see below) | 0..1 | Finer-grained nature of the experience. `intention` denotes prospective (future-directed) memory. |
| `tier` | enum `working` \| `session` \| `longterm` \| `archive` | 0..1 | Position in the memory hierarchy. Default `working`. Promotion is driven by age × importance × access. |
| `eventTime` | `Instant` (date-time) | 0..1 | WHEN the described event actually happened. MAY precede `createdAt`. |
| `importance` | `UnitInterval` ([0,1]) | 0..1 | Normalized importance score. |
| `confidence` | `Confidence` | 0..1 | Calibrated Bayesian Beta(α, β) posterior plus derived point estimate (`calibrated`). |
| `decay` | `Decay` | 0..1 | Forgetting state (`halfLifeHours`, `lastAccess`, `accessCount`, `anchored`). Anchored records resist decay. |
| `provenance` | `Provenance` | 0..1 | Origin and trust (`source`, `sourceType`, `credibility`, `externalId`). |
| `entityRefs` | array of `Reference` | 0..* | References to `Entity` resources mentioned by this memory. Enables spreading activation without a graph lookup. |
| `parentId` | `Id` (string) | 0..1 | Parent `MemoryRecord` id, for hierarchical knowledge trees. |
| `validUntil` | `Instant` (date-time) | 0..1 | Temporal invalidation: this record is considered superseded after this instant. |
| `version` | integer ≥ 1 | 0..1 | Record revision counter. Default `1`. |
| `extension` | array of `Extension` | 0..* | Typed, namespaced extensions carrying implementation-specific data without breaking core conformance. |

`additionalProperties` is `false`: a conformant `MemoryRecord` carries only the fields above.

### `experienceType` vocabulary

`conversation`, `decision`, `error`, `learning`, `discovery`, `pattern`, `context`, `task`, `code_edit`, `file_access`, `search`, `command`, `observation`, `intention`.

## Minimal

The required set is `resourceType`, `id`, `content`, `createdAt`.

```json
{
  "resourceType": "MemoryRecord",
  "id": "mem-001",
  "content": "Position OMIR as the at-rest format MCP transports, not a competing protocol.",
  "createdAt": "2026-05-30T11:42:05Z"
}
```

## Full

```json
{
  "resourceType": "MemoryRecord",
  "id": "mem-positioning",
  "meta": {
    "omirVersion": "R1",
    "profile": ["https://omir.io/spec/R1/profiles/agent-decision"],
    "source": "veld/0.7.6",
    "createdAt": "2026-05-30T11:42:05Z",
    "lastUpdated": "2026-05-30T11:42:05Z",
    "maturity": 4
  },
  "content": "Position OMIR as the at-rest data format that MCP/A2A transport, not a competing protocol.",
  "kind": "learning",
  "experienceType": "decision",
  "tier": "longterm",
  "createdAt": "2026-05-30T11:42:05Z",
  "eventTime": "2026-05-30T11:42:00Z",
  "importance": 0.95,
  "confidence": { "alpha": 9.0, "beta": 1.0, "calibrated": 0.9 },
  "decay": {
    "halfLifeHours": 8760,
    "lastAccess": "2026-05-30T11:42:05Z",
    "accessCount": 1,
    "anchored": true
  },
  "provenance": {
    "source": "design-review",
    "sourceType": "conversation",
    "credibility": 0.92,
    "externalId": "linear:OMIR-1"
  },
  "entityRefs": [
    { "ref": "Entity/omir" },
    { "ref": "Entity/varun" }
  ],
  "parentId": "mem-standardization-thread",
  "validUntil": "2027-05-30T00:00:00Z",
  "version": 1,
  "extension": [
    {
      "url": "https://veld.dev/omir/ext/scoring-signals",
      "valueJson": { "graphStrength": 0.84, "arousal": 0.6, "feedbackMomentum": 0.71 }
    }
  ]
}
```

## References

A `MemoryRecord` points to other resources by typed `Reference` (`ResourceType/id`):

- **`entityRefs[]` → `Entity`** — entities mentioned by this memory, used for spreading activation.

It also carries an intra-type link:

- **`parentId` → `MemoryRecord`** — parent record in a hierarchical knowledge tree (a local `Id`, not a `Reference`).

No `MemoryRecord` field references `Relationship`, `Episode`, or `Bundle` directly.
