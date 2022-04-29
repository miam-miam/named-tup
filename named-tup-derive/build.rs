use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};

use inwelling::Opts;

mod tup_finder;

pub fn main() {
    let mut all_identifiers = HashSet::new();

    // Docs.rs does not seem to like inwelling so we will just not use it.
    if env::var("DOCS_RS").is_err() {
        inwelling::inwelling(Opts {
            watch_manifest: true,
            watch_rs_files: true,
            dump_rs_paths: true,
        })
        .sections
        .into_iter()
        .for_each(|section| {
            section.rs_paths.unwrap().into_iter().for_each(|rs_path| {
                tup_finder::get_all_identifiers(&rs_path, &mut all_identifiers);
            })
        });
    }

    let mut all_identifiers: Vec<String> = all_identifiers.into_iter().collect();
    all_identifiers.sort();

    let out_dir = &env::var("OUT_DIR").unwrap();
    let gen_path = Path::new(out_dir).join("identifiers.in");

    let new_file_contents = format!("&{all_identifiers:?}");
    if should_rewrite_file(&gen_path, &new_file_contents) {
        fs::write(&gen_path, new_file_contents)
            .expect(&*format!("Could not write to file at: {gen_path:?}"));
    }
}

fn should_rewrite_file(gen_path: &Path, new_file_contents: &str) -> bool {
    if !gen_path.is_file() {
        return true;
    }
    let current_file_contents =
        fs::read_to_string(&gen_path).expect(&*format!("Could not read to file at: {gen_path:?}"));
    current_file_contents != new_file_contents
}
