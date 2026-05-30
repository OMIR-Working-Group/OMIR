# OMIR — Open Memory Interoperability Resources

**The open, vendor-neutral format for agent memory.**

OMIR (pronounced *"OH-meer"*) is an open, at-rest data format for portable AI agent and cognitive memory — a document standard, not a wire protocol or a product. Where MCP and A2A *transport* memory between agents and tools, OMIR *is* the memory at rest: a stable, inspectable file you can archive, diff, migrate, and hand to another system. It is modeled deliberately on HL7 FHIR, carrying that ecosystem's hard-won lessons about resources, references, profiles, and honest maturity grading into the agent-memory domain.

## Modeled on FHIR

| FHIR | OMIR |
|---|---|
| **F**ast | **O**pen |
| Healthcare | Memory |
| Interoperability Resources | Interoperability Resources |

> **Status:** OMIR R1 (draft) · Spec CC-BY-4.0 · Reference code Apache-2.0

## Repository layout

```
omir-standard/
├── schemas/          JSON Schema (draft 2020-12) — the normative field definitions
├── spec/             The OMIR specification (resource model, references, profiles, OMM)
├── examples/         Conformant sample documents, incl. minimal-bundle.omir
├── validator/        Reference validator (omir-validate CLI) — Apache-2.0
├── generators/       Code/schema generators and tooling — Apache-2.0
├── GOVERNANCE.md     Working Group, TSC, RFC + ballot process, deprecation policy
└── CONTRIBUTING.md   How to propose changes and participate
```

## Quickstart

```bash
# 1. Read a conformant OMIR document (a Bundle is the .omir file).
cat examples/minimal-bundle.omir

# 2. Validate it against the R1 schemas (omir-validate CLI — planned).
omir-validate examples/minimal-bundle.omir
```

A `.omir` file is canonical JSON / JSON-LD; `.omirb` is the compact binary profile (CBOR) for edge and robotics. Everything is a **Resource**; resources link by typed reference of the form `ResourceType/id`.

## Resources in R1

The R1 core is an 80/20 set plus a typed `extension[]` escape hatch for proprietary data. Each resource type is graded against the **OMIR Maturity Model (OMM)**, an integer 0–5 scale surfaced in `meta.maturity`:

- **MemoryRecord** — OMM-4 — the atomic unit of memory (experience, plan, prompt, learning); carries calibrated Bayesian confidence, multi-time-scale decay + anchoring, tier, provenance, and temporal invalidation.
- **Entity** — OMM-3 — a named thing extracted from memory (person, place, organization, concept, technology, …) with salience.
- **Relationship** — OMM-3 — a directed, weighted edge between two Entities; strength is dynamic under Hebbian plasticity, with temporal validity tracking.
- **Episode** — OMM-3 — a bounded experience; the episodic backbone from which records, entities, and relationships are derived.
- **Bundle** — the container that *is* the `.omir` document; a serialized collection of the resources above.

## Learn more

- [Specification](spec/) — the full R1 resource model, references, profiles, and the OMM.
- [GOVERNANCE.md](GOVERNANCE.md) — how OMIR is stewarded: the vendor-neutral Working Group, Technical Steering Committee, open RFC + ballot process, and deprecation policy.
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to get involved.

OMIR is convened by [Veld](https://github.com/Portll/veld) but owned by no single vendor; the standard is developed in the open by the OMIR Working Group.

---

**License.** The specification and schemas are licensed **CC-BY-4.0**. The reference code (validator, generators) is licensed **Apache-2.0**. A standard under a restrictive license is dead on arrival — the spec is free.
