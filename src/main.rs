use std::process::Command;

use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use rustdoc_types::Crate;

#[path = "../json_tests/tests.rs"]
pub(crate) mod tests;

#[derive(Debug, Clone, Copy)]
enum Version {
    Stage2,
    Nightly,
}

impl Version {
    fn specifier(&self) -> &'static str {
        match self {
            Version::Stage2 => "+stage2",
            Version::Nightly => "+nightly",
        }
    }
}

// TODO: Cache output
fn load_json(v: Version, file: &Utf8Path) -> Result<Crate> {
    let output_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("_output")
        .join(file)
        .join(v.specifier());

    let mut cmd = Command::new("rustdoc");
    cmd.arg(v.specifier())
        .arg(Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file))
        .args(["--output-format", "json"])
        .args(["-Z", "unstable-options"])
        .arg("--output")
        .arg(&output_dir);

    let output = cmd.output()?;

    if !output.status.success() {
        // TODO: Use get_args() to get the command line arguments
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        let args = cmd
            .get_args()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        bail!(
            "`rustdoc {}` failed with status {}\n=== stderr ===\n{}\n=== stdout===\n{}\n===",
            args,
            output.status,
            stderr,
            stdout
        );
    }

    let fname = file.file_name().unwrap().replace(".rs", ".json");

    let json = fs::File::open(output_dir.join(fname))?;
    let krate: Crate = serde_json::from_reader(json)?;

    return Ok(krate);
}

fn main() -> Result<()> {
    bail!("Usage: cargo test");
}
