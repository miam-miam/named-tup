extern crate core;

use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};

use inwelling::Opts;

mod tup_finder;

pub fn main() {
    let mut all_identifiers = HashSet::new();

    // Docs.rs does not seem to like inwelling so we will just not use it.
    if env::var("DOCS_RS").is_err() {
        let downstream = inwelling::collect_downstream(Opts {
            watch_manifest: true,
            watch_rs_files: false,
            dump_rs_paths: true,
        });

        downstream.packages.into_iter().for_each(|package| {
            if let Some(idents) = package.metadata.as_table().and_then(|t| t.get("arguments")) {
                if let Some(a) = idents.as_array() {
                    for ident in a {
                        if let Some(ident) = ident.as_str() {
                            all_identifiers.insert(ident.into());
                        } else {
                            panic!("Expected to find an array of idents in the Cargo.toml file.")
                        }
                    }
                } else {
                    panic!("Expected to find an array of idents in the Cargo.toml file.")
                }
            } else {
                package.rs_paths.unwrap().into_iter().for_each(|rs_path| {
                    println!("cargo:rerun-if-changed={}", rs_path.to_str().unwrap());
                    tup_finder::get_all_identifiers(&rs_path, &mut all_identifiers);
                })
            }
        });
    }

    // Add doc test identifiers
    if cfg!(feature = "add_dev_idents") {
        all_identifiers.extend(
            include! {"dev_idents.in"}
                .into_iter()
                .map(|s| s.to_string()),
        )
    }

    let mut all_identifiers: Vec<String> = all_identifiers.into_iter().collect();
    all_identifiers.sort();

    let out_dir = &env::var("OUT_DIR").unwrap();
    let gen_path = Path::new(out_dir).join("identifiers.in");

    let new_file_contents = format!("&{all_identifiers:?}");
    if should_rewrite_file(&gen_path, &new_file_contents) {
        fs::write(&gen_path, new_file_contents)
            .unwrap_or_else(|_| panic!("Could not write to file at: {gen_path:?}"));
    }
}

fn should_rewrite_file(gen_path: &Path, new_file_contents: &str) -> bool {
    if !gen_path.is_file() {
        return true;
    }
    let current_file_contents = fs::read_to_string(gen_path)
        .unwrap_or_else(|_| panic!("Could not read to file at: {gen_path:?}"));
    current_file_contents != new_file_contents
}
