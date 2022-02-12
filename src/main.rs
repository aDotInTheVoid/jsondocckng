use std::process::Command;

use anyhow::{bail, ensure, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use guard::guard_unwrap;
use rustdoc_types::{Crate, ItemEnum};

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

    let cmd = Command::new("rustdoc")
        .arg(v.specifier())
        .arg(
            Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join(file),
        )
        .args(["--output-format", "json"])
        .args(["-Z", "unstable-options"])
        .arg("--output")
        .arg(&output_dir)
        .output()?;

    if !cmd.status.success() {
        // TODO: Use get_args() to get the command line arguments
        let stderr = String::from_utf8_lossy(&cmd.stderr);
        let stdout = String::from_utf8_lossy(&cmd.stdout);
        bail!(
            "`rustdoc {}` failed with status {}\n--- stderr ---\n{}\n--- stdout---\n{}\n---",
            v.specifier(),
            cmd.status,
            stderr,
            stdout
        );
    }

    let json = fs::File::open(output_dir.join(file.with_extension("json")))?;
    let krate: Crate = serde_json::from_reader(json)?;

    return Ok(krate);
}

fn main() -> Result<()> {
    let h = load_json(Version::Nightly, "hello.rs".into())?;

    let hello = &h.index[&h.root];

    assert_eq!(hello.name.as_ref().unwrap(), "hello");
    assert_eq!(
        hello.docs.as_ref().unwrap(),
        "A crate that can print frendly greetings"
    );
    guard_unwrap!(let ItemEnum::Module(hmod) = &hello.inner);
    guard_unwrap!(let [hid] = &hmod.items[..]);
    let hello_fn_i = &h.index[&hid];
    assert_eq!(hello_fn_i.name.as_ref().unwrap(), "hello");
    assert_eq!(
        hello_fn_i.docs.as_ref().unwrap(),
        "Display a frendly greeting"
    );
    guard_unwrap!(let ItemEnum::Function(hfn) = &hello_fn_i.inner);
    assert_eq!(hfn.decl.inputs, []);
    assert_eq!(hfn.decl.output, None);

    Ok(())
}
