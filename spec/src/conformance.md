# Conformance

This section defines what it means for a document, and for an implementation, to
conform to OMIR R1. The key words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**,
and **MAY** are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

## Conformance levels

OMIR defines two levels of conformance.

### Core conformance

A document is **Core-conformant** when it is a valid [Bundle](./resources/bundle.md)
whose every resource validates against its R1 JSON Schema and satisfies the structural
rules below. Core conformance makes no claim about any domain profile.

### Profiled conformance

A document is **Profiled-conformant to profile P** when it is Core-conformant **and**
every resource that claims `P` in `meta.profile` satisfies all of `P`'s additional
constraints ([Profiles](./profiles.md)). Profiled conformance is always *in addition
to*, never *instead of*, core conformance.

## Document rules (MUST)

A Core-conformant document **MUST**:

1. **Be a Bundle.** The top-level object's `resourceType` is `Bundle` and
   `omirVersion` is `R1`. The `entry` array is present.
2. **Validate against the schemas.** Every entry validates against its resource schema
   (draft 2020-12) at `https://omir.io/spec/R1/schemas/`.
3. **Carry required fields per resource.** At minimum:
   - `MemoryRecord` — `resourceType`, `id`, `content`, `createdAt`.
   - `Entity` — `resourceType`, `id`, `name`.
   - `Relationship` — `resourceType`, `id`, `from`, `to`, `relationType`.
   - `Episode` — `resourceType`, `id`, `content`, `createdAt`.
4. **Use unique ids per type.** Each `id` matches `^[A-Za-z0-9._:-]{1,128}$` and is
   unique within its `resourceType` inside the Bundle.
5. **Satisfy reference integrity.** Every typed reference of the form `ResourceType/id`
   — in `entityRefs`, `Relationship.from` / `.to` / `.sourceEpisode`,
   `MemoryRecord.parentId` (within `MemoryRecord`) — **MUST** resolve to a resource of
   that type and id present in the same Bundle's `entry` array. A dangling reference is
   non-conformant.
6. **Carry no undeclared core fields.** Every R1 resource schema sets
   `additionalProperties: false`. Implementation-specific data **MUST** be carried in
   `extension[]` (see [Extensions](./extensions.md)), not as new top-level properties.
7. **Keep scores in range.** Every `UnitInterval` field (`importance`, `salience`,
   `strength`, `provenance.credibility`, `confidence.calibrated`) lies in `[0, 1]`.
8. **Use RFC 3339 timestamps.** Every `Instant` is a valid RFC 3339 / ISO 8601
   date-time.

## Producer / consumer rules

### Producers

A conforming **producer** (an implementation that emits OMIR):

- **MUST** emit Core-conformant Bundles.
- **MUST** place implementation-specific data in `extension[]` under a URL it controls,
  never under the `omir.io` namespace.
- **MUST** be able to emit the canonical `.omir` JSON form, even if it primarily emits
  `.omirb` ([Encodings](./encodings.md)).
- **SHOULD** populate `meta.source` and `meta.maturity` so consumers can reason about
  provenance and stability.
- **SHOULD NOT** emit dangling references; if a referenced resource is intentionally
  out of scope, the reference **SHOULD** be omitted rather than left dangling.

### Consumers

A conforming **consumer** (an implementation that reads OMIR):

- **MUST** accept any Core-conformant Bundle.
- **MUST** ignore unknown extensions and still process the resource. Encountering an
  unrecognized extension `url` **MUST NOT** cause rejection.
- **MUST** ignore unknown `meta.profile` URLs it does not implement, while still reading
  the core fields.
- **SHOULD** preserve extensions it does not understand when re-exporting, so data
  survives a read-modify-write round trip ("lossless pass-through").
- **MUST NOT** infer meaning from the *order* of entries; references resolve by
  `ResourceType/id`, not position.

## Declaring conformance

An implementation declares its conformance by publishing a short **conformance
statement** that lists:

1. the OMIR release it targets (R1);
2. its role(s): **producer**, **consumer**, or both;
3. the conformance level: **Core**, and any **Profiles** it implements (by URL);
4. for `.omirb` support, whether it produces, consumes, or both;
5. the canonical extension URLs it emits.

A producer **SHOULD** also stamp `meta.source` on the resources it emits (e.g.
`"veld/0.7.6"`) so a Bundle is self-describing.

### Self-test

Conformance is verifiable, not just asserted. An implementation **SHOULD** pass the
OMIR Working Group's reference validator (Apache-2.0) against:

- its emitted Bundles (producer conformance), and
- the published R1 example Bundles (consumer conformance).

The validator checks schema validity, reference integrity, id uniqueness, score
ranges, timestamp formats, and — when a profile URL is claimed — that profile's added
constraints.

## "Powered by OMIR" badge

An implementation that publishes a conformance statement **and** passes the reference
validator for the role(s) it claims **MAY** display the **"Powered by OMIR"** badge.

- The badge asserts **Core** conformance at minimum. An implementation **MAY** annotate
  the badge with the profiles it additionally satisfies (e.g. "Powered by OMIR —
  omir-coding-agent").
- The badge **MUST NOT** be displayed by an implementation that fails the reference
  validator for its claimed role, or that claims a profile it does not satisfy.
- The OMIR Working Group stewards the badge and its usage guidelines. Misrepresentation
  of conformance is grounds for the WG to request the badge's removal.
