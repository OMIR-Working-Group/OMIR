# Encodings

OMIR defines one logical data model and more than one byte-level encoding. The two
encodings **MUST** be lossless round-trips of the same model: a resource's meaning
**MUST NOT** depend on how it is encoded (see
[Design Principles §5](./principles.md#5-encoding-neutral)).

| Encoding | Extension | Media type (provisional) | Use |
|---|---|---|---|
| JSON / JSON-LD | `.omir` | `application/omir+json` | Canonical. Interchange, export, backup, hand-off. |
| Binary profile | `.omirb` | `application/omir+cbor` | Edge, robotics, high-volume. Compact, fast to parse. |

## JSON-LD canonical form

The canonical encoding is **JSON**, validated against the OMIR R1 JSON Schemas
(draft 2020-12) published at `https://omir.io/spec/R1/schemas/`. A conforming
`.omir` document is a [Bundle](./resources/bundle.md): a JSON object whose `entry`
array carries the resources.

```json
{
  "resourceType": "Bundle",
  "omirVersion": "R1",
  "id": "export-2026-05-30",
  "generatedAt": "2026-05-30T18:00:00Z",
  "source": "veld/0.7.6 (MIF adapter)",
  "entry": [
    {
      "resourceType": "MemoryRecord",
      "id": "m-001",
      "content": "User prefers execution-first responses, minimal hedging.",
      "createdAt": "2026-05-30T17:55:12Z",
      "kind": "learning",
      "tier": "longterm",
      "importance": 0.82,
      "confidence": { "alpha": 9.0, "beta": 1.0, "calibrated": 0.9 },
      "entityRefs": [{ "ref": "Entity/john" }]
    },
    {
      "resourceType": "Entity",
      "id": "john",
      "name": "John",
      "labels": ["person"],
      "salience": 0.7,
      "properNoun": true
    }
  ]
}
```

### JSON-LD compatibility

OMIR is **JSON-LD compatible** without forcing linked-data tooling on anyone. A Bundle
**MAY** carry an optional `@context`:

```json
{
  "@context": "https://omir.io/spec/R1/context.jsonld",
  "resourceType": "Bundle",
  "omirVersion": "R1",
  "entry": []
}
```

- The `@context` **SHOULD** be the canonical URL `https://omir.io/spec/R1/context.jsonld`,
  or an object that includes it.
- Adding `@context` **MUST NOT** change the core shape of any resource. A plain-JSON
  consumer that ignores `@context` reads exactly the same data as a JSON-LD processor.
- The `@context` maps OMIR property names to IRIs so that OMIR Bundles can participate
  in RDF graphs, SPARQL queries, and linked-data pipelines for implementations that
  want them. This is **opt-in**.
- `Episode.source` is mapped to its own predicate (`omir:episodeSource`, via a JSON-LD
  1.1 type-scoped context) so the *kind of input* an episode came from is not conflated
  with the producer/origin `source` carried by `Bundle`, `Meta`, and `Provenance`.
  Broader per-field disambiguation of shared term names arrives with the R2 vocabulary
  work (see [Toward a Global Standard](./global-standard.md)).

### Constraints on the canonical form

- Timestamps (`Instant`) **MUST** be RFC 3339 / ISO 8601. UTC is **RECOMMENDED**.
- Normalized scores (`UnitInterval`: `importance`, `salience`, `strength`,
  `credibility`, `confidence.calibrated`) **MUST** lie in the closed interval `[0, 1]`.
- Identifiers (`id`) **MUST** match `^[A-Za-z0-9._:-]{1,128}$` and **MUST** be unique
  within their `resourceType` inside a Bundle.
- Resources **MUST NOT** carry undeclared top-level properties; every R1 resource
  schema sets `additionalProperties: false`. Implementation-specific data goes in
  `extension[]` (see [Extensions](./extensions.md)).
- JSON numbers are **values, not lexical forms**: `9` and `9.0` denote the same
  `number`, and R1 does **not** mandate a canonical numeric spelling. A future binary or
  attestation profile may pin one (see
  [Toward a Global Standard](./global-standard.md)); within R1 the two are equivalent.

## The `.omirb` binary profile

For edge and robotics deployments — where an `.omir` JSON document is too large or too
slow to parse — OMIR defines a compact binary profile with the `.omirb` extension.

- `.omirb` is a **CBOR** (RFC 8949) encoding of the *same logical model* as the
  canonical JSON, with bincode permitted as an internal sub-profile for
  tightly-coupled producer/consumer pairs.
- The binary profile **MUST** be a lossless round-trip: decoding an `.omirb` document
  and re-encoding it as `.omir` JSON **MUST** yield a Bundle equivalent to the
  original (modulo insignificant ordering and whitespace).
- Field names, enums, and reference strings are preserved; the binary profile is a
  *re-encoding*, not a *re-modeling*. There are no binary-only fields and no
  JSON-only fields.
- The binary profile is **OPTIONAL**. An implementation that reads and writes only
  `.omir` JSON is fully conformant. An implementation that emits `.omirb` **MUST** be
  able to emit the equivalent `.omir` on request, so that the canonical form is always
  reachable.

### Choosing an encoding

- Use `.omir` for **interchange**: exports, backups, hand-offs between vendors,
  human inspection, version control.
- Use `.omirb` for **transport-adjacent persistence** on constrained devices: a robot
  writing memory to flash over a Zenoh link, an embedded agent with a tight parse
  budget. The `omir-robotics` profile ([Profiles](./profiles.md)) is built around this
  case.
