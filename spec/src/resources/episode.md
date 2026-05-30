# Episode

> **Generated artifact.** This page is generated from [`schemas/Episode.schema.json`](../../../schemas/Episode.schema.json) and [`schemas/common.schema.json`](../../../schemas/common.schema.json). Do not hand-edit; regenerate from the schema and the field tables will stay authoritative.

A bounded experience — the raw event from which `MemoryRecord`s, `Entity`s, and `Relationship`s are derived. The episodic backbone of the graph.

**Maturity:** OMM-3

## Purpose

An `Episode` is the raw, time-bounded input — a message, document, event, or observation — that downstream resources are extracted from. It preserves both event time and ingestion time, and lists the entities it produced, so derived records remain traceable to their origin.

## Fields

| Field | Type | Card. | Description |
|---|---|---|---|
| `resourceType` | `"Episode"` (const) | 1..1 | Discriminator. MUST be `Episode`. |
| `id` | `Id` (string) | 1..1 | Resource-local identifier, unique among `Episode` resources within a Bundle. Pattern `^[A-Za-z0-9._:-]{1,128}$`. |
| `content` | string | 1..1 | The actual experience data. |
| `createdAt` | `Instant` (date-time) | 1..1 | When this episode was ingested (ingestion time). |
| `meta` | `Meta` | 0..1 | Metadata envelope (`omirVersion`, `profile[]`, `source`, `createdAt`, `lastUpdated`, `maturity`). |
| `name` | string | 0..1 | Human-readable title for the episode. |
| `source` | enum `message` \| `document` \| `event` \| `observation` | 0..1 | What kind of input produced this episode. |
| `eventTime` | `Instant` (date-time) | 0..1 | When the original event occurred (event time). |
| `entityRefs` | array of `Reference` | 0..* | Entities extracted from this episode. |
| `metadata` | object (string → string) | 0..1 | Free-form string key/value metadata. |
| `extension` | array of `Extension` | 0..* | Typed, namespaced extensions carrying implementation-specific data without breaking core conformance. |

`additionalProperties` is `false`: a conformant `Episode` carries only the fields above.

> **Note on `source`.** On `Episode` this is an enum (`message` \| `document` \| `event` \| `observation`) and is distinct from the free-text `source` string inside `meta`/`Provenance` used on other resources.

## Minimal

The required set is `resourceType`, `id`, `content`, `createdAt`.

```json
{
  "resourceType": "Episode",
  "id": "ep-launch-chat",
  "content": "Varun and the agent agreed OMIR should be positioned as the at-rest format MCP transports, not a competitor to it.",
  "createdAt": "2026-05-30T11:42:03Z"
}
```

## Full

```json
{
  "resourceType": "Episode",
  "id": "ep-launch-chat",
  "meta": {
    "omirVersion": "R1",
    "source": "veld/0.7.6",
    "maturity": 3
  },
  "name": "Standardization discussion",
  "content": "Varun and the agent agreed OMIR should be positioned as the at-rest format MCP transports, not a competitor to it.",
  "source": "message",
  "eventTime": "2026-05-30T11:42:00Z",
  "createdAt": "2026-05-30T11:42:03Z",
  "entityRefs": [
    { "ref": "Entity/varun" },
    { "ref": "Entity/omir" }
  ],
  "metadata": { "channel": "design-review", "thread": "OMIR-positioning" },
  "extension": [
    {
      "url": "https://veld.dev/omir/ext/wavelet-session",
      "valueJson": { "sessionId": "sess-2026-05-30", "segment": 3 }
    }
  ]
}
```

## References

An `Episode` points to other resources by typed `Reference` (`ResourceType/id`):

- **`entityRefs[]` → `Entity`** — the entities extracted from this episode.

`Episode` is itself referenced by **`Relationship.sourceEpisode`**. No `Episode` field references `MemoryRecord`, `Relationship`, or `Bundle`.
