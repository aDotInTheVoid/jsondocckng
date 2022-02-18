#![cfg(test)]

use std::{io::BufReader, process::Command};

use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use rustdoc_types::{Crate, Item, Module};

#[path = "../json_tests/tests.rs"]
mod tests;

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

fn load_json(v: Version, file: &Utf8Path) -> Result<Crate> {
    let output_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("_output")
        .join(file)
        .join(v.specifier());
    let input_rs_file = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file);
    let fname = file.file_name().unwrap().replace(".rs", ".json");

    let output_json_file = output_dir.join(fname);

    let need_regen = match fs::metadata(&output_json_file) {
        Err(_) => true, // File doesn't exist
        Ok(json_meta) => {
            let rs_meta = fs::metadata(&input_rs_file)?;
            json_meta.modified()? < rs_meta.modified()?
        }
    };

    if need_regen {
        eprintln!("Regenerating {}", output_json_file);
        let mut cmd = Command::new("rustdoc");
        cmd.arg(v.specifier())
            .arg(input_rs_file)
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

            // TODO: Maybe invalidate json

            bail!(
                "`rustdoc {}` failed with status {}\n=== stderr ===\n{}\n=== stdout===\n{}\n===",
                args,
                output.status,
                stderr,
                stdout
            );
        }
    } else {
        eprintln!("Using cached {}", &output_json_file);
    }
    let json = fs::File::open(&output_json_file)?;
    let krate: Crate = serde_json::from_reader(BufReader::new(json))?;

    return Ok(krate);
}

fn load_by_name<'a>(c: &'a Crate, m: &Module, name: &str) -> &'a Item {
    for i in &m.items {
        let i = &c.index[&i];
        if i.name.as_deref() == Some(name) {
            return &i;
        }
    }
    panic!("No item named {}", name);
}

#[macro_export]
macro_rules! json_tests {
    ($($name:ident)*) => {
        paste::paste!{
        $(
            mod [<$name _test>];
            #[test]
            fn $name() {

                // [<$name>] will convert r#enum to enum
                let path = camino::Utf8PathBuf::from(file!()).parent().unwrap().join(stringify!([<$name>])).with_extension("rs");

                let krate = crate::load_json(
                    crate::Version::Nightly, &path
                ).unwrap();
                [<$name _test>]::test(krate);
            }
        )*
    }
    }
}
