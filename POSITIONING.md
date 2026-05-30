# OMIR — Positioning & Announcement

**OMIR — Open Memory Interoperability Resources** (pronounced "OH-meer"). An open, vendor-neutral, at-rest data format for portable AI agent memory. Modeled on HL7 FHIR. Spec under CC-BY-4.0; reference code under Apache-2.0.

*For technical founders and engineering leaders.*

---

## 1. The gap

The agent-memory field is saturated with **products and engines**, and starved of a **format**.

Mem0 just raised $24M. MemoryLake is shipping a proprietary "memory passport." There are six-plus distinct products called some flavor of "Engram," plus DeepSeek's Engram technique. Every serious agent stack now has a memory layer — and every one of them stores that memory in a closed, internal shape.

So the obvious question has no answer: **how do you move an agent's memory from one system to another?** Today you can't, except by writing a bespoke adapter per pair of vendors. There is no neutral file you can hand to an auditor, archive for ten years, diff in code review, or feed to a competitor's engine. The market has many memory *databases* and zero memory *documents*. That absence is the opening.

## 2. The wedge

OMIR is not a better memory engine. It is the **format every memory engine writes to.**

The parallel is HL7 FHIR. FHIR did not win by building a better hospital or a better EHR. It won by defining the at-rest resources — `Patient`, `Observation`, `Encounter` — that every hospital, every vendor, every regulator reads and writes. The product layer stayed competitive; the *interchange* layer became shared infrastructure. OMIR makes the same bet for agent memory: don't be the better engine, be the thing every engine can export.

A standard wins on neutrality, not features. So the spec is governed by a vendor-neutral **OMIR Working Group** in a neutral org — Veld convenes, Veld does not own — and the spec and schemas are **CC-BY-4.0**, deliberately decoupled from any contributor's commercial license. A standard under a restrictive license is dead on arrival.

## 3. How it works

OMIR is FHIR-shaped. Everything is a typed **Resource**; resources link by typed reference of the form `ResourceType/id`. The R1 core is small on purpose — `MemoryRecord`, `Entity`, `Relationship`, `Episode`, and a `Bundle` container that *is* the `.omir` document. The core covers the **80%** every memory system shares: content, timestamps, tiers, calibrated Bayesian confidence, multi-time-scale decay and anchoring, Hebbian relationship strength, temporal invalidation, provenance and source credibility, prospective ("intention") memory. The proprietary **20%** rides in a typed `extension[]` escape hatch — a vendor's bespoke scoring signals travel inside the file without breaking conformance, and any reader may safely ignore extensions it doesn't understand. Maturity is graded honestly per resource type via the **OMIR Maturity Model (OMM)**, surfaced in `meta.maturity`: `MemoryRecord` is OMM-4; `Entity`, `Relationship`, and `Episode` are OMM-3. We do not overclaim stability.

## 4. The honest risk

Two failure modes, stated plainly.

**Standards die without a second implementer.** A format authored by exactly one vendor is just that vendor's export format with a logo. If OMIR has one producer and zero independent consumers, it is not a standard — it is documentation for a private file. The single most important early metric is not spec completeness; it is *a second implementer who did not write the spec.*

**MCP and A2A could absorb memory portability.** MCP and A2A (Anthropic and Google, both now under the Linux Foundation) are the gravitational center of agent interoperability. It is entirely plausible that one of them bolts a "memory portability" annex onto a transport protocol and calls the question closed — at which point an external at-rest format looks redundant.

## 5. The mitigation

Both risks point to the same posture: **scope OMIR narrowly, and stand inside the tent, not against it.**

MCP and A2A *transport* memory between agents at runtime. OMIR *is* the memory at rest — the document on disk, in the archive, in the export. These are complementary, not competing; a protocol annex describing how to *ship* a memory blob still benefits from a neutral, versioned, schema-validated definition of what that blob *contains*. OMIR is the natural payload format for exactly the annex that would otherwise threaten it.

So the move is to pitch OMIR **into** the Linux Foundation's Agentic AI ecosystem, not against it — as the at-rest complement to the MCP/A2A wire layer. The same neutrality that makes a standard credible also makes it adoptable by the body that owns the protocols. We want to be the format MCP/A2A reach for, not the format they route around.

## 6. The ask

Two concrete commitments, in order:

1. **Convene the Working Group.** Stand up the vendor-neutral OMIR Working Group in a neutral GitHub org with a public RFC + ballot process, a Technical Steering Committee, and a published deprecation policy. Veld seeds R1 and the reference validator; governance is shared from day one.
2. **Secure the first external implementer.** One independent team that *reads or writes* `.omir` without having authored the spec. That single integration is what converts a file format into a standard. Everything else is in service of getting there.

---

**OMIR: MCP and A2A move the memory. OMIR is the memory.**
