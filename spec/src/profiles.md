# Profiles

The OMIR core is intentionally permissive: almost every field is optional so that any
memory system can map onto it. That generality is a problem for any *particular*
domain, where a consumer needs guarantees — "every record will have provenance," "every
entity will carry an embedding," "decay state will always be present."

A **Profile** closes that gap. A profile is a published constraint on the OMIR core:

> **A profile = a constrained subset of the core + a set of required extensions.**

Specifically, a profile **MAY**:

- mark otherwise-optional core fields as **required**;
- **forbid** core fields or enum values that are meaningless in its domain;
- **require** specific extensions (by `url`) to be present;
- tighten value ranges or cardinalities within what the base schema allows.

A profile **MUST NOT**:

- add new top-level core fields (use extensions);
- relax a base-schema requirement (a profile only ever *narrows*);
- contradict the base schema (a profiled resource is always also a valid base
  resource).

A resource declares the profiles it claims via `meta.profile`, an array of canonical
profile URLs. A Bundle is **profile-conformant** when every resource that claims a
profile satisfies that profile's constraints (see [Conformance](./conformance.md)).

R1 ships three reference profiles.

## `omir-coding-agent`

**URL:** `https://omir.io/spec/R1/profile/omir-coding-agent`

For coding agents (Claude Code, Copilot agents, Cursor) that remember decisions,
patterns, edits, and project context.

Constraints:

- `MemoryRecord.provenance` is **REQUIRED**, and `provenance.sourceType` **SHOULD** be
  one of `conversation`, `document`, `command`, or `observation`.
- `MemoryRecord.kind` is **REQUIRED** (one of `memory`, `plan`, `prompt`, `learning`).
- For records whose `experienceType` is `code_edit`, `file_access`, or `command`,
  `provenance.externalId` **SHOULD** be present (e.g. `github:pr-123`,
  `linear:SHO-39`) for traceability back to the artifact.
- `Entity.labels` **SHOULD** include `technology`, `project`, or `skill` where
  applicable, so a project graph is reconstructable.

Required extensions: none mandated; tool-call telemetry, if carried, **SHOULD** use a
vendor extension URL.

## `omir-robotics` (edge / Zenoh)

**URL:** `https://omir.io/spec/R1/profile/omir-robotics`

For embodied agents and robots persisting memory on constrained edge hardware, often
over a Zenoh/ROS2 transport.

Constraints:

- The `.omirb` binary encoding ([Encodings](./encodings.md)) is **RECOMMENDED** for
  persistence under this profile; producers **MUST** still be able to emit `.omir`
  JSON on request.
- `Episode.source` **SHOULD** be `observation` or `event` (robots ingest sensor and
  event streams, not chat messages).
- `MemoryRecord.eventTime` is **REQUIRED** — physical events are timestamped at
  occurrence, distinct from ingestion time.
- A spatial extension is **REQUIRED** on location-bearing records and entities, under a
  canonical URL (e.g. `https://omir.io/spec/R1/extension/spatial-pose`) carrying a
  pose/frame payload in `valueJson`. Spatial coordinates are **not** core in R1.
- `decay.halfLifeHours` **SHOULD** be present so on-device forgetting survives power
  cycles.

## `omir-personal-assistant`

**URL:** `https://omir.io/spec/R1/profile/omir-personal-assistant`

For personal-assistant agents that hold long-lived facts and preferences about a single
user, and must handle PII responsibly.

Constraints:

- `MemoryRecord.confidence` is **REQUIRED** — a personal assistant must know how sure
  it is before acting on a remembered fact.
- `MemoryRecord.validUntil` **SHOULD** be set on facts known to be time-bounded (a
  current address, an employer), so stale facts are detectable.
- Prospective memory is supported: records with `experienceType: "intention"` carry
  future-directed reminders and **MUST** be filtered out of ordinary recall by a
  conforming consumer.
- A PII-classification extension is **REQUIRED** on records and entities that may carry
  personal data, under a canonical URL (e.g.
  `https://omir.io/spec/R1/extension/pii-class`), so downstream handling can honor it.
- `Entity.salience` **SHOULD** be present so the assistant can rank what matters to its
  user.

## Defining your own profile

Implementations and domains **MAY** publish additional profiles. A profile definition
**MUST**:

1. have a stable canonical URL;
2. state the base resource(s) it constrains;
3. enumerate its added requirements, forbidden fields/values, and required extension
   URLs;
4. guarantee that any resource conforming to the profile is also a valid base R1
   resource.

Profiles are versioned with the release (R1, R2, …) and follow the same deprecation
policy as the core.
