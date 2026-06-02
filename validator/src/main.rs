//! `omir-validate` CLI — a thin front end over the [`omir_validate`] library.
//!
//! Validate the bundled example at Core:
//!     omir-validate ../examples/minimal-bundle.omir
//! Canonical JSON report for CI / badge automation:
//!     omir-validate bundle.omir --format json -o report.json

use std::io::{IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use omir_validate::{validate, Level, Outcome, Report, SchemaFiles, Schemas};

#[derive(Parser)]
#[command(
    name = "omir-validate",
    version,
    about = "Reference conformance validator for the OMIR R1 at-rest memory format."
)]
struct Args {
    /// One or more .omir documents to validate.
    #[arg(required = true, value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Conformance level to grade against.
    #[arg(long, value_enum, default_value_t = LevelArg::Core)]
    level: LevelArg,

    /// Report rendering.
    #[arg(long, value_enum, default_value_t = FormatArg::Human)]
    format: FormatArg,

    /// Write the report to a file instead of stdout.
    #[arg(short = 'o', long, value_name = "PATH")]
    out: Option<PathBuf>,

    /// Load a profile definition (repeatable). Not implemented in this build.
    #[arg(long, value_name = "URL|PATH")]
    profile: Vec<String>,

    /// Require the named profile on eligible resources. Not implemented yet.
    #[arg(long = "require-profile", value_name = "URL")]
    require_profile: Vec<String>,

    /// Override the embedded R1 schema set with schemas from this directory.
    #[arg(long = "schema-dir", value_name = "DIR")]
    schema_dir: Option<PathBuf>,

    /// Disable ANSI color in human output.
    #[arg(long = "no-color")]
    no_color: bool,

    /// Suppress the per-check summary; print only RESULT and findings.
    #[arg(short = 'q', long)]
    quiet: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum LevelArg {
    Core,
    Strict,
    Profile,
}

#[derive(Clone, Copy, ValueEnum)]
enum FormatArg {
    Human,
    Json,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if !args.profile.is_empty()
        || !args.require_profile.is_empty()
        || matches!(args.level, LevelArg::Profile)
    {
        eprintln!(
            "note: profile conformance is not implemented in omir-validate {} (scaffold); \
             profile checks are reported as skipped and never affect Core conformance.",
            env!("CARGO_PKG_VERSION")
        );
    }

    let level = match args.level {
        LevelArg::Core => Level::Core,
        LevelArg::Strict => Level::Strict,
        LevelArg::Profile => Level::Profile,
    };

    let files = match &args.schema_dir {
        Some(dir) => match SchemaFiles::from_dir(dir) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::from(2);
            }
        },
        None => SchemaFiles::embedded(),
    };
    let schemas = match Schemas::build(&files) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to compile OMIR R1 schemas: {e}");
            return ExitCode::from(2);
        }
    };

    let mut reports: Vec<Report> = Vec::new();
    let mut worst: u8 = 0; // 0 = pass, 1 = fail, 2 = tool/usage error

    for file in &args.files {
        if file.extension().and_then(|e| e.to_str()) == Some("omirb") {
            eprintln!(
                "error: {}: binary .omirb input is not supported in omir-validate {} (JSON .omir only).",
                file.display(),
                env!("CARGO_PKG_VERSION")
            );
            worst = worst.max(2);
            continue;
        }
        let text = match std::fs::read_to_string(file) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("error: cannot read {}: {e}", file.display());
                worst = worst.max(2);
                continue;
            }
        };
        let doc: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error (E000): {}: not valid JSON: {e}", file.display());
                worst = worst.max(2);
                continue;
            }
        };
        let report = validate(&file.display().to_string(), &doc, level, &schemas);
        if report.result == Outcome::Fail {
            worst = worst.max(1);
        }
        reports.push(report);
    }

    let color = !args.no_color && args.out.is_none() && std::io::stdout().is_terminal();

    let rendered = match args.format {
        FormatArg::Human => reports
            .iter()
            .map(|r| render_human_quiet(r, color, args.quiet))
            .collect::<Vec<_>>()
            .join("\n"),
        FormatArg::Json => {
            if reports.len() == 1 {
                serde_json::to_string_pretty(&reports[0]).unwrap_or_default()
            } else {
                serde_json::to_string_pretty(&reports).unwrap_or_default()
            }
        }
    };

    if !rendered.is_empty() {
        match &args.out {
            Some(path) => {
                if let Err(e) = std::fs::write(path, format!("{rendered}\n")) {
                    eprintln!("error: cannot write {}: {e}", path.display());
                    return ExitCode::from(2);
                }
            }
            None => {
                let mut out = std::io::stdout();
                let _ = writeln!(out, "{rendered}");
            }
        }
    }

    ExitCode::from(worst)
}

/// In quiet mode, drop everything before the `findings:`/`RESULT:` block.
fn render_human_quiet(report: &Report, color: bool, quiet: bool) -> String {
    let full = report.render_human(color);
    if !quiet {
        return full;
    }
    let mut out = String::new();
    let mut keep = false;
    for line in full.lines() {
        if line.starts_with("findings:") || line.starts_with("RESULT:") {
            keep = true;
        }
        if keep {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.trim_end().to_string()
}
