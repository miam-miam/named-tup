use std::collections::HashSet;
use std::env;
use std::fs::{read_dir, read_to_string, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use pest::*;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TupFinder;

fn main() {
    println!("rerun-if-changed=src");

    let files = get_rust_files();
    let all_identifiers = get_all_identifiers(files);
    let new_file_contents = create_file_contents(all_identifiers);

    let out_dir = &env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(out_dir).join("tuple_types.rs");

    if dest_path.is_file() {
        let mut tuple_types = File::open(&dest_path).unwrap();

        let mut old_file_contents = String::new();
        tuple_types.read_to_string(&mut old_file_contents).unwrap();

        if new_file_contents != old_file_contents {
            drop(tuple_types);
            let mut f_dest = File::create(&dest_path).unwrap();
            f_dest.write_all(new_file_contents.as_ref()).unwrap();
        }
    } else {
        let mut tuple_types = File::create(&dest_path).unwrap();
        tuple_types.write_all(new_file_contents.as_ref()).unwrap();
    }
}

fn create_file_contents(all_identifiers: HashSet<String>) -> String {
    let mut identifiers: Vec<String> = all_identifiers.into_iter().collect();

    identifiers.sort();
    let joined = identifiers.join(",");

    format!(
        "pub fn message() -> &'static str {{
            \"{joined}\"
        }}"
    )
}

fn get_rust_files() -> Vec<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut directories = vec![manifest_dir.join("src")];
    let mut files = vec![];

    while let Some(directory) = directories.pop() {
        for entry in read_dir(directory).unwrap() {
            let path = entry.unwrap().path();

            if path.is_dir() {
                directories.push(path);
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    files.push(path);
                }
            }
        }
    }
    files
}

fn get_all_identifiers(files: Vec<PathBuf>) -> HashSet<String> {
    let mut all_identifiers = HashSet::new();
    for file in files {
        let code = read_to_string(file).unwrap();

        let tuples = TupFinder::parse(Rule::file, &code);

        if let Ok(tuples) = tuples {
            for tuple in tuples {
                let ident = tuple.as_str();
                all_identifiers.insert(ident.to_string());
            }
        }
    }
    all_identifiers
}
