//! Embedded OMIR R1 JSON Schemas and the compiled validators built from them.
//!
//! The validator embeds a pinned copy of the canonical schemas at build time
//! (`include_str!` of the repository's single-source-of-truth `schemas/`), so
//! it runs fully offline with no network `$ref` resolution. `--schema-dir`
//! overrides the embedded set for testing against a draft revision.

use jsonschema::{Registry, Validator};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

/// The four closed R1 resource types that may appear in `Bundle.entry[]`.
pub const RESOURCE_TYPES: [&str; 4] = ["MemoryRecord", "Entity", "Relationship", "Episode"];

/// Canonical `$id` of the shared definitions schema; relative `$ref`s in every
/// resource schema resolve against this.
const COMMON_ID: &str = "https://omir.io/spec/R1/schemas/common.schema.json";

/// The six R1 schema documents as text. Either the embedded copies or a set
/// loaded from `--schema-dir`.
pub struct SchemaFiles {
    pub common: String,
    pub bundle: String,
    pub memory_record: String,
    pub entity: String,
    pub relationship: String,
    pub episode: String,
}

impl SchemaFiles {
    /// The schemas pinned into the binary at build time.
    pub fn embedded() -> Self {
        SchemaFiles {
            common: include_str!("../../schemas/common.schema.json").to_string(),
            bundle: include_str!("../../schemas/Bundle.schema.json").to_string(),
            memory_record: include_str!("../../schemas/MemoryRecord.schema.json").to_string(),
            entity: include_str!("../../schemas/Entity.schema.json").to_string(),
            relationship: include_str!("../../schemas/Relationship.schema.json").to_string(),
            episode: include_str!("../../schemas/Episode.schema.json").to_string(),
        }
    }

    /// Load the schema set from a directory (`--schema-dir`).
    pub fn from_dir(dir: &Path) -> Result<Self, String> {
        let read = |name: &str| -> Result<String, String> {
            let path = dir.join(name);
            std::fs::read_to_string(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))
        };
        Ok(SchemaFiles {
            common: read("common.schema.json")?,
            bundle: read("Bundle.schema.json")?,
            memory_record: read("MemoryRecord.schema.json")?,
            entity: read("Entity.schema.json")?,
            relationship: read("Relationship.schema.json")?,
            episode: read("Episode.schema.json")?,
        })
    }
}

/// Compiled validators: one for the Bundle envelope and one per resource type.
pub struct Schemas {
    /// Validates the Bundle container (required fields, `additionalProperties`,
    /// `entry` is an array) — but NOT the entries themselves; per-entry
    /// validation is done by dispatch so errors are attributable.
    pub envelope: Validator,
    /// `resourceType` -> compiled validator for that resource schema.
    pub by_type: BTreeMap<String, Validator>,
}

impl Schemas {
    pub fn build(files: &SchemaFiles) -> Result<Schemas, String> {
        let common: Value = parse(&files.common, "common.schema.json")?;

        // Register `common` once in a referencing Registry so each resource
        // schema's relative `$ref`s resolve offline against COMMON_ID — no
        // network retrieval. The Registry is consumed into each compiled
        // Validator, so it need not outlive this function.
        let registry = Registry::new()
            .add(COMMON_ID, common.clone())
            .map_err(|e| format!("registering common.schema.json: {e}"))?
            .prepare()
            .map_err(|e| format!("preparing schema registry: {e}"))?;

        // Compile a root schema against the shared registry. Formats
        // (date-time, uri) are asserted, matching README §2.1.
        let compile = |root: &Value| -> Result<Validator, String> {
            jsonschema::options()
                .should_validate_formats(true)
                .with_registry(&registry)
                .build(root)
                .map_err(|e| e.to_string())
        };

        // Envelope: take Bundle.schema.json but neutralize the parts owned by
        // dedicated checks — `entry.items` (per-entry dispatch handles those)
        // and the `resourceType`/`omirVersion` consts (E111 / E300 give clearer
        // messages than a generic schema violation).
        let mut bundle: Value = parse(&files.bundle, "Bundle.schema.json")?;
        bundle["properties"]["entry"]["items"] = json!({ "type": "object" });
        bundle["properties"]["resourceType"] = json!({ "type": "string" });
        bundle["properties"]["omirVersion"] = json!({ "type": "string" });
        let envelope = compile(&bundle)?;

        let mut by_type = BTreeMap::new();
        for (name, src) in [
            ("MemoryRecord", &files.memory_record),
            ("Entity", &files.entity),
            ("Relationship", &files.relationship),
            ("Episode", &files.episode),
        ] {
            let schema = parse(src, name)?;
            by_type.insert(name.to_string(), compile(&schema)?);
        }

        Ok(Schemas { envelope, by_type })
    }
}

fn parse(src: &str, what: &str) -> Result<Value, String> {
    serde_json::from_str(src).map_err(|e| format!("invalid JSON in {what}: {e}"))
}
