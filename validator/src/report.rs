//! Conformance report types and rendering.
//!
//! The JSON form ([`Report`] serialized) is the canonical product consumed by
//! CI and badge automation; the human form ([`Report::render_human`]) is a
//! convenience rendering of the same underlying data. Both come from one
//! `Report` value, so they can never disagree.

use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;

/// Validator version, surfaced in every report.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Conformance level the document is graded against.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Core,
    Strict,
    Profile,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Level::Core => "core",
            Level::Strict => "strict",
            Level::Profile => "profile",
        })
    }
}

/// Severity of a single finding.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Which check group produced a finding.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Check {
    Structural,
    ReferenceIntegrity,
    VersionPresence,
    Profile,
}

/// Pass / fail / skipped status of a check group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Fail,
    Skipped,
}

/// Overall outcome at the requested level.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Pass,
    Fail,
}

/// Locates the entry a finding belongs to (`null` for Bundle-level findings).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryRef {
    pub index: usize,
    pub resource_type: String,
    pub id: String,
}

/// A single conformance finding. Stable `code`, a severity, a JSON Pointer
/// locating the offending node, and a human message.
#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    pub code: &'static str,
    pub severity: Severity,
    pub check: Check,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<EntryRef>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    pub message: String,
    /// Whether this finding fails *Core* conformance, independent of the level
    /// the run was graded at. Not serialized — it only drives `coreConformant`.
    #[serde(skip)]
    pub breaks_core: bool,
}

impl Finding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: &'static str,
        severity: Severity,
        check: Check,
        entry: Option<EntryRef>,
        path: String,
        value: Option<Value>,
        message: String,
        breaks_core: bool,
    ) -> Self {
        Finding {
            code,
            severity,
            check,
            entry,
            path,
            value,
            message,
            breaks_core,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub entries_total: usize,
    pub entries_by_type: BTreeMap<String, usize>,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Checks {
    pub structural: CheckStatus,
    pub reference_integrity: CheckStatus,
    pub version_presence: CheckStatus,
    pub profile: CheckStatus,
}

/// The canonical conformance report. Field order and names match
/// `validator/README.md` §5.1.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub report: &'static str,
    pub omir_validate_version: &'static str,
    pub schema_release: &'static str,
    pub target: String,
    pub level: Level,
    pub result: Outcome,
    pub summary: Summary,
    pub checks: Checks,
    pub findings: Vec<Finding>,
    pub core_conformant: bool,
    pub badge_eligible: bool,

    // Human-render-only counters; excluded from the canonical JSON.
    #[serde(skip)]
    pub refs_total: usize,
    #[serde(skip)]
    pub refs_resolved: usize,
    #[serde(skip)]
    pub entries_with_meta_version: usize,
    #[serde(skip)]
    pub bundle_version: Option<String>,
}

impl Report {
    /// Assemble a report from the raw findings produced by the check groups.
    #[allow(clippy::too_many_arguments)]
    pub fn assemble(
        target: &str,
        level: Level,
        entries: &[Value],
        findings: Vec<Finding>,
        refs_total: usize,
        refs_resolved: usize,
        entries_with_meta_version: usize,
        bundle_version: Option<String>,
        resource_types: &[&str],
    ) -> Report {
        let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
        for e in entries {
            if let Some(rt) = e.get("resourceType").and_then(Value::as_str) {
                if resource_types.contains(&rt) {
                    *by_type.entry(rt.to_string()).or_default() += 1;
                }
            }
        }

        let errors = findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
        let warnings = findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .count();
        let info = findings
            .iter()
            .filter(|f| f.severity == Severity::Info)
            .count();

        let group_status = |c: Check| -> CheckStatus {
            if findings
                .iter()
                .any(|f| f.check == c && f.severity == Severity::Error)
            {
                CheckStatus::Fail
            } else {
                CheckStatus::Pass
            }
        };

        let checks = Checks {
            structural: group_status(Check::Structural),
            reference_integrity: group_status(Check::ReferenceIntegrity),
            version_presence: group_status(Check::VersionPresence),
            profile: CheckStatus::Skipped,
        };

        // Any error fails. At strict, any warning fails too (no warnings tolerated).
        let fail = errors > 0 || (level == Level::Strict && warnings > 0);
        let result = if fail { Outcome::Fail } else { Outcome::Pass };

        // Core conformance is level-independent: zero findings that break Core.
        let core_conformant = !findings.iter().any(|f| f.breaks_core);

        Report {
            report: "OMIRConformanceReport",
            omir_validate_version: VERSION,
            schema_release: "R1",
            target: target.to_string(),
            level,
            result,
            summary: Summary {
                entries_total: entries.len(),
                entries_by_type: by_type,
                errors,
                warnings,
                info,
            },
            checks,
            findings,
            core_conformant,
            badge_eligible: core_conformant,
            refs_total,
            refs_resolved,
            entries_with_meta_version,
            bundle_version,
        }
    }

    fn entries_by_type_desc(&self) -> String {
        self.summary
            .entries_by_type
            .iter()
            .map(|(k, n)| format!("{n} {k}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Render the report for a terminal. `color` toggles ANSI styling.
    pub fn render_human(&self, color: bool) -> String {
        let p = Palette::new(color);
        let mut o = String::new();

        o.push_str(&format!(
            "omir-validate {} — OMIR R1 conformance\n",
            self.omir_validate_version
        ));
        o.push_str(&format!(
            "target: {}   level: {}\n\n",
            self.target, self.level
        ));

        let entries_desc = self.entries_by_type_desc();
        let entries_detail = if entries_desc.is_empty() {
            String::new()
        } else {
            format!(": {entries_desc}")
        };
        o.push_str(&format!(
            "  {} {}   ({} entries{})\n",
            dotted("structural"),
            status_word(self.checks.structural, &p),
            self.summary.entries_total,
            entries_detail,
        ));
        o.push_str(&format!(
            "  {} {}   ({} of {} references resolved)\n",
            dotted("reference integrity"),
            status_word(self.checks.reference_integrity, &p),
            self.refs_resolved,
            self.refs_total,
        ));
        let bundle_v = self.bundle_version.as_deref().unwrap_or("absent");
        o.push_str(&format!(
            "  {} {}   (Bundle={}, {}/{} entries meta.omirVersion=R1)\n",
            dotted("version presence"),
            status_word(self.checks.version_presence, &p),
            bundle_v,
            self.entries_with_meta_version,
            self.summary.entries_total,
        ));
        o.push_str(&format!(
            "  {} {}\n\n",
            dotted("profile"),
            p.dim("skipped (profile checks not run)"),
        ));

        if !self.findings.is_empty() {
            o.push_str("findings:\n");
            for f in &self.findings {
                let loc = match &f.entry {
                    Some(er) if !er.resource_type.is_empty() => {
                        format!("{}/{} (entry {})", er.resource_type, er.id, er.index)
                    }
                    Some(er) => format!("entry {}", er.index),
                    None => "Bundle".to_string(),
                };
                o.push_str(&format!(
                    "  {}  {}  {}\n",
                    sev_tag(f.severity, &p),
                    p.bold(&format!("{:<5}", f.code)),
                    loc
                ));
                o.push_str(&format!("        {}\n", f.message));
                o.push_str(&format!("        {} {}\n", p.dim("at"), p.dim(&f.path)));
            }
            o.push('\n');
        }

        o.push_str(&self.result_line(&p));
        o
    }

    fn result_line(&self, p: &Palette) -> String {
        match self.result {
            Outcome::Pass => p.green(&format!(
                "RESULT: PASS  —  Core conformant ✓  ({})",
                if self.badge_eligible {
                    "badge-eligible"
                } else {
                    "badge withheld"
                }
            )),
            Outcome::Fail => {
                let tail = if self.core_conformant {
                    "Core conformant, but failed at this level".to_string()
                } else {
                    "NOT Core conformant (badge withheld)".to_string()
                };
                p.red(&format!(
                    "RESULT: FAIL  —  {} error(s), {} warning(s)  —  {}",
                    self.summary.errors, self.summary.warnings, tail
                ))
            }
        }
    }
}

fn dotted(label: &str) -> String {
    // "structural ........." padded with dots to a fixed column.
    format!("{:.<24}", format!("{label} "))
}

fn status_word(s: CheckStatus, p: &Palette) -> String {
    match s {
        CheckStatus::Pass => p.green("pass"),
        CheckStatus::Fail => p.red("FAIL"),
        CheckStatus::Skipped => p.dim("skipped"),
    }
}

fn sev_tag(s: Severity, p: &Palette) -> String {
    match s {
        Severity::Error => p.red("error  "),
        Severity::Warning => p.yellow("warning"),
        Severity::Info => p.blue("info   "),
    }
}

/// Minimal ANSI palette — no external color dependency.
struct Palette {
    on: bool,
}

impl Palette {
    fn new(on: bool) -> Self {
        Palette { on }
    }
    fn wrap(&self, code: &str, s: &str) -> String {
        if self.on {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn green(&self, s: &str) -> String {
        self.wrap("32", s)
    }
    fn red(&self, s: &str) -> String {
        self.wrap("31", s)
    }
    fn yellow(&self, s: &str) -> String {
        self.wrap("33", s)
    }
    fn blue(&self, s: &str) -> String {
        self.wrap("34", s)
    }
    fn dim(&self, s: &str) -> String {
        self.wrap("2", s)
    }
    fn bold(&self, s: &str) -> String {
        self.wrap("1", s)
    }
}
