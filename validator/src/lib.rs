//! `omir-validate` core library.
//!
//! Decides whether a `.omir` document is a conformant OMIR R1 Bundle, and to
//! what level, returning a [`Report`]. The CLI ([`main`](../main.rs)) is a thin
//! front end over [`validate`]; producers can embed this same logic in-process
//! so a CLI run and a self-check always agree.
//!
//! Checks run as four ordered groups (see `validator/README.md` §2):
//! 1. structural (envelope + per-entry schema dispatch),
//! 2. reference integrity (every `ResourceType/id` resolves within the Bundle),
//! 3. version presence (`omirVersion == "R1"`),
//! 4. profile (opt-in; reported as skipped in this build).

pub mod report;
pub mod schemas;

pub use report::{
    Check, CheckStatus, Checks, EntryRef, Finding, Level, Outcome, Report, Severity, Summary,
    VERSION,
};
pub use schemas::{SchemaFiles, Schemas, RESOURCE_TYPES};

use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Validate a parsed `.omir` document and produce a conformance report.
pub fn validate(target: &str, doc: &Value, level: Level, schemas: &Schemas) -> Report {
    let mut findings: Vec<Finding> = Vec::new();

    // ---------- 2.1 Structural: Bundle envelope ----------
    for err in schemas.envelope.iter_errors(doc) {
        let p = err.instance_path().to_string();
        let path = if p.is_empty() { "/".to_string() } else { p };
        findings.push(Finding::new(
            "E110",
            Severity::Error,
            Check::Structural,
            None,
            path,
            None,
            format!("Bundle envelope: {err}"),
            true,
        ));
    }
    // Dedicated resourceType check (clearer than a generic const violation).
    match doc.get("resourceType").and_then(Value::as_str) {
        Some("Bundle") => {}
        other => findings.push(Finding::new(
            "E111",
            Severity::Error,
            Check::Structural,
            None,
            "/resourceType".to_string(),
            other.map(Value::from),
            format!("Bundle.resourceType MUST be \"Bundle\"{}.", found(other)),
            true,
        )),
    }

    let entries: Vec<Value> = doc
        .get("entry")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    // ---------- 2.1 Structural: per-entry dispatch ----------
    for (i, entry) in entries.iter().enumerate() {
        if !entry.is_object() {
            findings.push(Finding::new(
                "E100",
                Severity::Error,
                Check::Structural,
                None,
                format!("/entry/{i}"),
                None,
                format!("entry[{i}] is not a JSON object."),
                true,
            ));
            continue;
        }
        let id = entry
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        match entry.get("resourceType").and_then(Value::as_str) {
            Some(rt) if RESOURCE_TYPES.contains(&rt) => {
                let validator = schemas
                    .by_type
                    .get(rt)
                    .expect("a validator exists for every known resource type");
                let eref = EntryRef {
                    index: i,
                    resource_type: rt.to_string(),
                    id: id.clone(),
                };
                for err in validator.iter_errors(entry) {
                    findings.push(Finding::new(
                        "E120",
                        Severity::Error,
                        Check::Structural,
                        Some(eref.clone()),
                        format!("/entry/{i}{}", err.instance_path()),
                        None,
                        format!("{rt}: {err}"),
                        true,
                    ));
                }
            }
            other => {
                let eref = EntryRef {
                    index: i,
                    resource_type: other.unwrap_or("").to_string(),
                    id,
                };
                findings.push(Finding::new(
                    "E101",
                    Severity::Error,
                    Check::Structural,
                    Some(eref),
                    format!("/entry/{i}/resourceType"),
                    other.map(Value::from),
                    format!(
                        "entry[{i}].resourceType {} is not one of the four R1 resource types \
                         (MemoryRecord, Entity, Relationship, Episode).",
                        found(other)
                    ),
                    true,
                ));
            }
        }
    }

    // ---------- 2.2 Reference integrity ----------
    let (refs_total, refs_resolved) = reference_integrity(&entries, level, &mut findings);

    // ---------- 2.3 Version presence ----------
    let bundle_version = doc
        .get("omirVersion")
        .and_then(Value::as_str)
        .map(str::to_string);
    let entries_with_meta_version = version_presence(doc, &entries, level, &mut findings);

    // ---------- 2.4 Profile: opt-in, not implemented in this build (skipped) ----------

    Report::assemble(
        target,
        level,
        &entries,
        findings,
        refs_total,
        refs_resolved,
        entries_with_meta_version,
        bundle_version,
        &RESOURCE_TYPES,
    )
}

/// Shape of a reference-bearing field.
#[derive(Clone, Copy)]
enum RefShape {
    /// `{ "ref": "Type/id" }`
    ObjectSingle,
    /// `[ { "ref": "Type/id" }, … ]`
    ObjectArray,
    /// a bare local id string whose target type is implied (e.g. `parentId`)
    BareId,
}

/// One closed-world reference rule: a field on a resource type, its shape, and the
/// resource type(s) its target may have. `targets` is a SET to admit poly-typed
/// references in later releases (e.g. a PROV `derivedFrom` over any resource). This
/// table is the single source of truth for the reference walk — new reference fields
/// are added here, not as bespoke walk code. Seeded with the exact R1 closed-world set
/// (Conformance CR-5).
struct RefRule {
    rt: &'static str,
    field: &'static str,
    shape: RefShape,
    targets: &'static [&'static str],
}

const REF_RULES: &[RefRule] = &[
    RefRule {
        rt: "MemoryRecord",
        field: "entityRefs",
        shape: RefShape::ObjectArray,
        targets: &["Entity"],
    },
    RefRule {
        rt: "MemoryRecord",
        field: "parentId",
        shape: RefShape::BareId,
        targets: &["MemoryRecord"],
    },
    RefRule {
        rt: "Episode",
        field: "entityRefs",
        shape: RefShape::ObjectArray,
        targets: &["Entity"],
    },
    RefRule {
        rt: "Relationship",
        field: "from",
        shape: RefShape::ObjectSingle,
        targets: &["Entity"],
    },
    RefRule {
        rt: "Relationship",
        field: "to",
        shape: RefShape::ObjectSingle,
        targets: &["Entity"],
    },
    RefRule {
        rt: "Relationship",
        field: "sourceEpisode",
        shape: RefShape::ObjectSingle,
        targets: &["Episode"],
    },
];

/// Build the `(resourceType, id)` index, flag duplicates, then confirm every
/// typed reference resolves to an entry of the correct type. Returns
/// `(references_walked, references_resolved)`.
fn reference_integrity(
    entries: &[Value],
    level: Level,
    findings: &mut Vec<Finding>,
) -> (usize, usize) {
    let mut index: HashSet<(String, String)> = HashSet::new();
    let mut ids_by_type: HashMap<&'static str, Vec<String>> = HashMap::new();

    for (i, e) in entries.iter().enumerate() {
        let rt = e.get("resourceType").and_then(Value::as_str);
        let id = e.get("id").and_then(Value::as_str);
        if let (Some(rt), Some(id)) = (rt, id) {
            if let Some(canon) = RESOURCE_TYPES.iter().find(|t| **t == rt) {
                if !index.insert((rt.to_string(), id.to_string())) {
                    // Duplicate (resourceType, id): references to it are ambiguous.
                    let severity = if level == Level::Strict {
                        Severity::Error
                    } else {
                        Severity::Warning
                    };
                    findings.push(Finding::new(
                        "W202",
                        severity,
                        Check::ReferenceIntegrity,
                        Some(EntryRef {
                            index: i,
                            resource_type: rt.to_string(),
                            id: id.to_string(),
                        }),
                        format!("/entry/{i}/id"),
                        Some(Value::from(id)),
                        format!(
                            "Duplicate id: more than one {rt} shares id \"{id}\"; \
                             references to it are ambiguous."
                        ),
                        false, // tolerated at Core
                    ));
                }
                ids_by_type.entry(*canon).or_default().push(id.to_string());
            }
        }
    }

    let mut walk = RefWalk {
        index: &index,
        ids_by_type: &ids_by_type,
        findings,
        total: 0,
        resolved: 0,
    };

    for (i, e) in entries.iter().enumerate() {
        let self_rt = e.get("resourceType").and_then(Value::as_str).unwrap_or("");
        let self_id = e.get("id").and_then(Value::as_str).unwrap_or("");
        for rule in REF_RULES.iter().filter(|r| r.rt == self_rt) {
            walk.walk_rule(e, rule, i, self_rt, self_id);
        }
    }

    (walk.total, walk.resolved)
}

/// Carries the index and counters through the reference walk.
struct RefWalk<'a> {
    index: &'a HashSet<(String, String)>,
    ids_by_type: &'a HashMap<&'static str, Vec<String>>,
    findings: &'a mut Vec<Finding>,
    total: usize,
    resolved: usize,
}

impl RefWalk<'_> {
    /// Walk one reference-bearing field according to its declared shape, resolving
    /// each reference against the closed-world index.
    fn walk_rule(&mut self, e: &Value, rule: &RefRule, i: usize, self_rt: &str, self_id: &str) {
        let field = rule.field;
        let Some(v) = e.get(field) else { return };
        match rule.shape {
            RefShape::ObjectSingle => {
                if let Some(s) = v.get("ref").and_then(Value::as_str) {
                    if let Some((rt, rid)) = s.split_once('/') {
                        self.resolve(
                            rt,
                            rid,
                            rule.targets,
                            format!("/entry/{i}/{field}/ref"),
                            Value::from(s),
                            i,
                            self_rt,
                            self_id,
                        );
                    }
                }
            }
            RefShape::ObjectArray => {
                if let Some(arr) = v.as_array() {
                    for (j, item) in arr.iter().enumerate() {
                        if let Some(s) = item.get("ref").and_then(Value::as_str) {
                            if let Some((rt, rid)) = s.split_once('/') {
                                self.resolve(
                                    rt,
                                    rid,
                                    rule.targets,
                                    format!("/entry/{i}/{field}/{j}/ref"),
                                    Value::from(s),
                                    i,
                                    self_rt,
                                    self_id,
                                );
                            }
                        }
                    }
                }
            }
            RefShape::BareId => {
                if let Some(s) = v.as_str() {
                    // A bare id implies its target type, so a type mismatch is impossible.
                    self.resolve(
                        rule.targets[0],
                        s,
                        rule.targets,
                        format!("/entry/{i}/{field}"),
                        Value::from(s),
                        i,
                        self_rt,
                        self_id,
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve(
        &mut self,
        ref_type: &str,
        ref_id: &str,
        targets: &'static [&'static str],
        path: String,
        raw: Value,
        i: usize,
        self_rt: &str,
        self_id: &str,
    ) {
        self.total += 1;
        let holder = EntryRef {
            index: i,
            resource_type: self_rt.to_string(),
            id: self_id.to_string(),
        };
        let exists = self
            .index
            .contains(&(ref_type.to_string(), ref_id.to_string()));
        if exists {
            if targets.contains(&ref_type) {
                self.resolved += 1;
            } else {
                let expected = if targets.len() == 1 {
                    format!("a {}", targets[0])
                } else {
                    format!("one of {}", targets.join(", "))
                };
                self.findings.push(Finding::new(
                    "E201",
                    Severity::Error,
                    Check::ReferenceIntegrity,
                    Some(holder),
                    path,
                    Some(raw),
                    format!("Type mismatch: reference resolves to a {ref_type}, but {expected} is required here."),
                    true,
                ));
            }
        } else {
            let hint = self
                .suggest(targets[0], ref_id)
                .map(|s| format!(" Did you mean \"{}/{s}\"?", targets[0]))
                .unwrap_or_default();
            self.findings.push(Finding::new(
                "E200",
                Severity::Error,
                Check::ReferenceIntegrity,
                Some(holder),
                path,
                Some(raw),
                format!("Dangling reference: \"{ref_type}/{ref_id}\" does not resolve to any entry in the Bundle.{hint}"),
                true,
            ));
        }
    }

    /// Closest id of the expected type within edit distance 2 (typo hint).
    fn suggest(&self, expected: &str, target: &str) -> Option<String> {
        self.ids_by_type
            .get(expected)
            .into_iter()
            .flatten()
            .map(|id| (levenshtein(id, target), id))
            .filter(|(d, id)| *d > 0 && *d <= 2 && *d < id.len().max(target.len()))
            .min_by_key(|(d, _)| *d)
            .map(|(_, id)| id.clone())
    }
}

/// `omirVersion == "R1"` at the Bundle level and on every entry that carries a
/// `meta`. Returns how many entries declare `meta.omirVersion == "R1"`.
fn version_presence(
    doc: &Value,
    entries: &[Value],
    level: Level,
    findings: &mut Vec<Finding>,
) -> usize {
    match doc.get("omirVersion").and_then(Value::as_str) {
        Some("R1") => {}
        other => findings.push(Finding::new(
            "E300",
            Severity::Error,
            Check::VersionPresence,
            None,
            "/omirVersion".to_string(),
            other.map(Value::from),
            format!("Bundle.omirVersion MUST be \"R1\"{}.", found(other)),
            true,
        )),
    }

    let mut with_version = 0usize;
    for (i, e) in entries.iter().enumerate() {
        let rt = e
            .get("resourceType")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let id = e
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let holder = || {
            Some(EntryRef {
                index: i,
                resource_type: rt.clone(),
                id: id.clone(),
            })
        };

        match e.get("meta") {
            None => {
                if level == Level::Strict {
                    findings.push(Finding::new(
                        "E302",
                        Severity::Error,
                        Check::VersionPresence,
                        holder(),
                        format!("/entry/{i}/meta"),
                        None,
                        "At --level strict, every entry MUST carry a meta block with omirVersion \"R1\".".to_string(),
                        false, // strict-only; does not break Core
                    ));
                } else {
                    findings.push(Finding::new(
                        "I302",
                        Severity::Info,
                        Check::VersionPresence,
                        holder(),
                        format!("/entry/{i}"),
                        None,
                        "No meta block; consider adding meta.omirVersion \"R1\" so the entry is self-describing.".to_string(),
                        false,
                    ));
                }
            }
            Some(meta) => match meta.get("omirVersion").and_then(Value::as_str) {
                Some("R1") => with_version += 1,
                other => findings.push(Finding::new(
                    "E301",
                    Severity::Error,
                    Check::VersionPresence,
                    holder(),
                    format!("/entry/{i}/meta/omirVersion"),
                    other.map(Value::from),
                    format!(
                        "meta.omirVersion MUST be \"R1\" when meta is present{}.",
                        found(other)
                    ),
                    true,
                )),
            },
        }
    }
    with_version
}

/// Render a found value for a message: ` (found "x")` or ` (absent)`.
fn found(v: Option<&str>) -> String {
    match v {
        Some(s) => format!(" (found \"{s}\")"),
        None => " (absent)".to_string(),
    }
}

/// Classic Levenshtein edit distance (small ids; allocation-light).
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, &ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            cur[j + 1] = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}
