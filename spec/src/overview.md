# Overview

**OMIR** — *Open Memory Interoperability Resources* (pronounced "OH-meer") — is an
open, vendor-neutral data format for portable AI agent and cognitive memory. This
document describes **OMIR R1**, the first release.

OMIR is modeled, deliberately and openly, on **HL7 FHIR** (Fast Healthcare
Interoperability Resources). Where FHIR made clinical data portable across hospital
systems, OMIR makes *memory* portable across agent systems. The mapping is direct:
*Open* ↔ *Fast*, *Memory* ↔ *Healthcare*, and "Interoperability Resources" is kept
verbatim.

## What OMIR is

OMIR is an **at-rest data format** — a document/file standard. It defines how a unit
of agent memory looks when it is written to disk, exported, backed up, or handed from
one system to another.

OMIR is:

- **A file format.** The canonical serialization is JSON / JSON-LD with the `.omir`
  extension; a compact binary profile uses `.omirb`.
- **Vendor-neutral.** No single implementation owns the format. Veld convenes the
  working group as the reference implementation, but the spec and schemas are free
  (see [Conformance](./conformance.md) and the governance notes below).
- **Resource-oriented.** Borrowing FHIR's model, everything in OMIR is a *Resource*.

OMIR is **not**:

- **Not a wire protocol.** OMIR does not define how memory is transported, queried
  live, or synchronized between running agents. It describes the bytes at rest.
- **Not a product.** OMIR is a standard. Implementations are products; the format
  is not.
- **Not a database.** OMIR says nothing about indexing, vector search, or storage
  engines. Those are implementation concerns.

## Where OMIR sits: MCP, A2A, and OMIR

Two transport standards already exist for agent systems, both now stewarded by the
Linux Foundation:

- **MCP** (Model Context Protocol, originally Anthropic) — connects agents to tools
  and context sources.
- **A2A** (Agent-to-Agent, originally Google) — lets agents talk to each other.

Both are **transport** concerns: they move data between live endpoints. Neither
defines what memory *is* once it has been written down.

> **MCP and A2A transport memory. OMIR *is* the memory at rest.**

OMIR is **complementary, never competing**. An agent can serve memory over MCP, hand
it to a peer over A2A, and persist or export it as an `.omir` Bundle. The transport
moves the bytes; OMIR defines the bytes. A clean separation of "in motion" from
"at rest" is exactly the separation FHIR drew between its RESTful API (motion) and its
Resources (rest).

## The resource model

Every piece of data in OMIR is a **Resource**. R1 defines five resource types:

| Resource | Purpose | OMM |
|---|---|---|
| [MemoryRecord](./resources/memoryrecord.md) | The atomic unit of memory: a remembered experience, plan, prompt, or learning. | 4 |
| [Entity](./resources/entity.md) | A named thing extracted from memory — person, place, concept, technology, skill. | 3 |
| [Relationship](./resources/relationship.md) | A directed, weighted (Hebbian) edge between two entities. | 3 |
| [Episode](./resources/episode.md) | A bounded experience — the raw event the other resources are derived from. | 3 |
| [Bundle](./resources/bundle.md) | The container. A Bundle **is** the `.omir` document. | — |

The maturity column refers to the **OMIR Maturity Model (OMM)** — see
[Design Principles](./principles.md#honest-maturity-via-omm). It is surfaced per
resource in `meta.maturity`.

## References

Resources do not nest each other arbitrarily; they **link** by typed reference. A
reference is an object carrying a single string of the form `ResourceType/id`:

```json
{ "ref": "Entity/john" }
```

The reference target's `ResourceType` MUST be one of `MemoryRecord`, `Entity`,
`Relationship`, or `Episode`, and the `id` MUST match an `id` of that type. References
resolve **within the Bundle**: a `MemoryRecord` carrying `entityRefs: [{ "ref":
"Entity/john" }]` is satisfied by an `Entity` with `id: "john"` in the same Bundle's
`entry` array. Reference integrity is a conformance rule
([Conformance](./conformance.md)).

This is how the graph is expressed at rest: a `Relationship` points `from` one Entity
`to` another; an `Episode` lists the `entityRefs` it produced; a `MemoryRecord` links
the entities it mentions. No pointers, no foreign keys — just typed string references.

## The 80/20 rule

OMIR does not try to model every field every memory system has ever invented. It
follows FHIR's **80/20 rule**: the *core* of each resource captures the ~80% of fields
that ~80% of implementations actually need — content, timestamps, importance,
confidence, decay, provenance, references. The remaining long tail of
implementation-specific data does not bloat the core.

Instead, every resource carries a typed `extension[]` array: a structured,
namespaced escape hatch for proprietary or experimental data. Veld's 20-signal
retrieval scores, external isotropy/closure dimensions, and name embeddings all ride
in extensions — never in the core. A consumer that does not understand an extension
**MUST** ignore it and still process the resource. See [Extensions](./extensions.md).

Where an implementation needs to *constrain* the core (require certain fields, forbid
others, mandate specific extensions), it publishes a **Profile**. See
[Profiles](./profiles.md).

## Heritage

OMIR's core faithfully carries the cognitive-memory features proven in its reference
implementation, **Veld — Agentic Memory**:

- **Calibrated Bayesian confidence** — a Beta(α, β) posterior plus a point estimate,
  not a bare float.
- **Multi-time-scale decay and anchoring** — "better forgetting": records decay on a
  half-life unless anchored.
- **Tiered memory** — `working → session → longterm → archive`.
- **Hebbian relationship strength** — edges strengthen with co-activation, decay
  without use.
- **Temporal invalidation** — `validUntil` on records, `invalidatedAt` on
  relationships.
- **Provenance and source credibility** — where a memory came from, and how much to
  trust the origin.
- **Prospective memory** — `experienceType: "intention"` for future-directed records.
- **Entity salience** — how strongly an entity pulls on retrieval.

These are not optional add-ons bolted onto a flat record; they are the core OMIR
resource fields. The format encodes *how memory behaves over time*, not just *what
was said*.

## Governance and licensing

OMIR is stewarded by a vendor-neutral **OMIR Working Group** in a neutral GitHub
organization. Veld convenes the group but does not own the standard. The group
operates an open RFC + ballot process, a Technical Steering Committee (TSC), semantic
release versioning (R1, R2, …), and a published deprecation policy.

Licensing is deliberately decoupled from any implementation:

- The **specification and schemas** are licensed **CC-BY-4.0**.
- The **reference code** (validator, generators) is licensed **Apache-2.0**.

A standard under a restrictive license is dead on arrival. The OMIR spec is, and will
remain, free.
