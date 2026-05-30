# Design Principles

The OMIR design is governed by six principles. The key words **MUST**, **MUST NOT**,
**SHOULD**, **SHOULD NOT**, and **MAY** in this specification are to be interpreted as
described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

## 1. Open and neutral

OMIR is an open standard owned by no vendor.

- The specification and JSON Schemas **MUST** remain freely licensed (CC-BY-4.0 for
  the spec, Apache-2.0 for the reference code). A standard behind a restrictive
  license is dead on arrival.
- No single implementation's behavior is normative. Where this spec describes
  cognitive behavior (decay, Hebbian strengthening, tiering), it describes the
  *data shape that records that behavior*, not a required algorithm.
- The format **MUST NOT** privilege one vendor's identifiers, URLs, or schemas in its
  core. Vendor-specific data lives in `extension[]` under vendor-controlled URLs.

## 2. At-rest, not a protocol

OMIR describes memory **at rest** — bytes on disk, in a backup, in an export, in a
hand-off file.

- OMIR **MUST NOT** be read as a transport or query protocol. It defines no endpoints,
  no request/response shapes, no synchronization semantics.
- Live transport is the job of MCP and A2A. OMIR is complementary: those protocols
  move memory; OMIR *is* the memory once it lands. An implementation **MAY** serve an
  OMIR Bundle over any transport, but the Bundle's validity does not depend on how it
  travelled.

## 3. 80/20 core plus extensions

Each resource defines a small, stable core and an open extension mechanism.

- The **core** of a resource captures the ~80% of fields that ~80% of memory systems
  need. New core fields are added conservatively and only through the ballot process.
- Implementation-specific, experimental, or proprietary data **MUST** be carried in
  the typed `extension[]` array under a canonical URL, **not** by adding properties to
  the core. Every resource schema sets `additionalProperties: false`, so undeclared
  top-level fields are non-conformant by construction.
- A consumer **MUST** ignore extensions it does not recognize and **MUST** still
  process the resource. Unknown extensions are never an error. See
  [Extensions](./extensions.md).

## 4. Honest maturity via OMM

OMIR grades the stability of each resource **type** with the **OMIR Maturity Model
(OMM)**, an integer 0–5 surfaced in `meta.maturity`.

| OMM | Meaning |
|---|---|
| 0 | Draft — shape may change without notice. |
| 1 | Early — implemented, but field set is volatile. |
| 2 | Trial use — implemented in more than one system; breaking changes possible. |
| 3 | Stabilizing — widely implemented; changes require deprecation. |
| 4 | Mature — broadly implemented, battle-tested; backwards-compatible evolution only. |
| 5 | Normative — locked; changes only via a new release. |

R1 grades honestly and **MUST NOT** overclaim:

- `MemoryRecord` — **OMM-4**.
- `Entity`, `Relationship`, `Episode` — **OMM-3**.
- New resources introduced in later releases start lower and earn their level through
  real, independent implementation.

Maturity is a *promise about change*, not a quality score. A consumer **SHOULD** treat
lower-OMM resources as more likely to change between releases.

## 5. Encoding-neutral

OMIR defines a single logical data model with more than one byte-level encoding.

- The canonical encoding is **JSON / JSON-LD** (`.omir`), chosen for ubiquity and
  human-readability.
- A compact **binary profile** (`.omirb`, CBOR/bincode) exists for edge and robotics
  deployments where size and parse cost matter.
- The two encodings **MUST** be lossless round-trips of the same logical model. A
  resource's *meaning* **MUST NOT** depend on its encoding. See
  [Encodings](./encodings.md).

The file extensions `.omir` and `.omirb` are the canonical, collision-free identifiers
for the format. Implementations **MUST NOT** use `.mf` or `.mif` for OMIR documents;
those extensions are heavily collided and are not part of this standard.

## 6. Forgetting is first-class

Most data formats assume data is permanent. Memory is not. OMIR treats *forgetting* as
core, not as an afterthought.

- Every `MemoryRecord` **MAY** carry a `decay` block: a half-life, last-access time,
  access count, and an `anchored` flag. Anchored records resist decay.
- `Relationship` strength is **Hebbian**: it rises with co-activation and falls without
  use. The `strength` field records the current synaptic weight.
- Temporal **invalidation** is explicit: `validUntil` supersedes a record after an
  instant; `invalidatedAt` retires a relationship.
- `tier` (`working → session → longterm → archive`) records where a memory sits in the
  consolidation hierarchy.

These fields let a faithful implementation reconstruct *better forgetting* from an
at-rest document — decay curves, anchors, and tier transitions survive export. OMIR
does not mandate a forgetting *algorithm*; it standardizes the *state* that algorithm
reads and writes.
