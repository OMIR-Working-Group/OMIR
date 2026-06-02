# Invalid fixtures

Each file here is a **deliberately non-conformant** OMIR R1 Bundle. The reference
validator (`validator/`) MUST reject every one of them. They are the negative
half of the conformance corpus — `../minimal-bundle.omir` is the positive case.

| Fixture | Defect | Expected finding |
|---|---|---|
| `dangling-ref.omir` | `Relationship.to` points at `Entity/ghost`, which is not in the Bundle. | `E200` (dangling reference) |
| `score-out-of-range.omir` | `MemoryRecord.importance` is `1.5`; `UnitInterval` is `[0,1]`. | `E120` (structural) |
| `bad-enum.omir` | `MemoryRecord.tier` is `"permanent"`, not in the `tier` enum. | `E120` (structural) |
| `unknown-field.omir` | A proprietary top-level `vendorScore` field; resources are `additionalProperties: false`. | `E120` (structural) |
| `dangling-parent-id.omir` | `MemoryRecord.parentId` (a bare id) points at a `MemoryRecord` not in the Bundle. | `E200` (dangling reference) |
| `dangling-source-episode.omir` | `Relationship.sourceEpisode` points at an `Episode` not in the Bundle. | `E200` (dangling reference) |

Run one:

```
cd ../../validator
cargo run -- ../examples/invalid/dangling-ref.omir        # exits 1
```
