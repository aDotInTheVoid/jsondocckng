#![cfg(test)]

use std::{io::BufReader, process::Command};

use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use from_item::FromItem;
use fs_err as fs;
use rustdoc_types::{Crate, Id, Item, Module};

use crate::from_item::IntoKind;

mod from_item;
#[path = "../json_tests/tests.rs"]
mod tests;
mod validate;

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
                let tcrate = crate::TCrate::new(krate);
                [<$name _test>]::test(tcrate);
            }
        )*
    }
    }
}

struct TCrate {
    krate: Crate,
}

impl TCrate {
    fn new(krate: Crate) -> Self {
        let this = Self { krate };
        this.validate();
        this
    }

    fn validate(&self) {
        // TODO: Reimplement https://github.com/rust-lang/rust/blob/master/src/etc/check_missing_items.py
        for (id, item) in &self.krate.index {
            assert_eq!(id.0, item.id.0);
            self.validate_item(item);
        }
    }

    fn load_root<T: FromItem>(&self, name: &str) -> &T {
        self.load_item_by_name(self.root(), name).into_kind()
    }

    fn load_root_id(&self, name: &str) -> Id {
        self.load_item_by_name(self.root(), name).id.clone()
    }

    fn load_root_item(&self, name: &str) -> &Item {
        self.load_item_by_name(self.root(), name)
    }

    fn root(&self) -> &Module {
        Module::from_item(self.root_item())
    }

    fn root_item(&self) -> &Item {
        &self.krate.index[&self.krate.root]
    }

    fn load_item_by_name(&self, m: &Module, name: &str) -> &Item {
        // TODO: Check no duplicate names

        for i in &m.items {
            let i = &self.krate.index[&i];
            if i.name.as_deref() == Some(name) {
                return &i;
            }
        }
        // TODO: Better error
        panic!("No item named {}", name);
    }
}
