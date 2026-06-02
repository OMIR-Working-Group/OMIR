//! Integration tests: validate the published corpus through the embedded
//! schema set. The conformant example must pass Core (and Strict); every
//! invalid fixture must be rejected with the expected finding code.

use std::path::PathBuf;

use omir_validate::{validate, Level, Outcome, SchemaFiles, Schemas};

fn schemas() -> Schemas {
    Schemas::build(&SchemaFiles::embedded()).expect("embedded R1 schemas compile")
}

fn load(rel: &str) -> serde_json::Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn has_code(report: &omir_validate::Report, code: &str) -> bool {
    report.findings.iter().any(|f| f.code == code)
}

#[test]
fn minimal_bundle_is_core_conformant() {
    let report = validate(
        "minimal",
        &load("../examples/minimal-bundle.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(
        report.result,
        Outcome::Pass,
        "unexpected findings: {:?}",
        report.findings
    );
    assert!(report.core_conformant);
    assert!(report.badge_eligible);
    assert_eq!(report.summary.entries_total, 5);
    assert_eq!(report.refs_total, 7);
    assert_eq!(report.refs_resolved, 7);
}

#[test]
fn minimal_bundle_passes_strict() {
    let report = validate(
        "minimal",
        &load("../examples/minimal-bundle.omir"),
        Level::Strict,
        &schemas(),
    );
    assert_eq!(
        report.result,
        Outcome::Pass,
        "unexpected findings: {:?}",
        report.findings
    );
}

#[test]
fn dangling_reference_is_rejected() {
    let report = validate(
        "dangling",
        &load("../examples/invalid/dangling-ref.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(!report.core_conformant);
    assert!(
        has_code(&report, "E200"),
        "expected E200, got: {:?}",
        report.findings
    );
}

#[test]
fn score_out_of_range_is_rejected() {
    let report = validate(
        "score",
        &load("../examples/invalid/score-out-of-range.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(
        has_code(&report, "E120"),
        "expected E120, got: {:?}",
        report.findings
    );
}

#[test]
fn bad_enum_is_rejected() {
    let report = validate(
        "enum",
        &load("../examples/invalid/bad-enum.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(
        has_code(&report, "E120"),
        "expected E120, got: {:?}",
        report.findings
    );
}

#[test]
fn unknown_top_level_field_is_rejected() {
    let report = validate(
        "unknown",
        &load("../examples/invalid/unknown-field.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(
        has_code(&report, "E120"),
        "expected E120, got: {:?}",
        report.findings
    );
}

// Guard fixtures for the registry-driven reference walker (REMEDIATION C1): the
// bare-id `parentId` and the pre-existing `Relationship.sourceEpisode` are the two
// closed-world fields most at risk of being dropped in the refactor. They MUST still
// be rejected when dangling.
#[test]
fn dangling_parent_id_is_rejected() {
    let report = validate(
        "parent",
        &load("../examples/invalid/dangling-parent-id.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(
        has_code(&report, "E200"),
        "expected E200, got: {:?}",
        report.findings
    );
}

#[test]
fn dangling_source_episode_is_rejected() {
    let report = validate(
        "sourceEpisode",
        &load("../examples/invalid/dangling-source-episode.omir"),
        Level::Core,
        &schemas(),
    );
    assert_eq!(report.result, Outcome::Fail);
    assert!(
        has_code(&report, "E200"),
        "expected E200, got: {:?}",
        report.findings
    );
}
