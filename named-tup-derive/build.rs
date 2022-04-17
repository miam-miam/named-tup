use std::path::{Path, PathBuf};
use std::{env, fs};

mod tup_finder;

pub fn main() {
    let files = get_rust_files();
    let all_identifiers = tup_finder::get_all_identifiers(files);

    let out_dir = &env::var("OUT_DIR").unwrap();
    dbg!(&out_dir);
    let gen_path = Path::new(out_dir).join("identifiers.in");
    dbg!(&gen_path);

    let new_file_contents = create_file_contents(all_identifiers);
    if should_rewrite_file(&gen_path, &new_file_contents) {
        fs::write(&gen_path, new_file_contents).unwrap();
    }
}

fn should_rewrite_file(gen_path: &Path, new_file_contents: &str) -> bool {
    if !gen_path.is_file() {
        return true;
    }
    let current_file_contents = fs::read_to_string(&gen_path).unwrap();
    current_file_contents != new_file_contents
}

fn create_file_contents(all_identifiers: Vec<String>) -> String {
    let joined = all_identifiers.join("\",\"");
    dbg!(&joined);

    format!("[\"{joined}\"]")
}

fn get_rust_files() -> Vec<PathBuf> {
    let src = env::var("NAMED_TUPS_DIR").expect(
        "Environment variable NAMED_TUPS_DIR is not set, please set it to your src directory.",
    );
    println!("cargo:rerun-if-changed={src}");
    let mut directories = vec![PathBuf::new().join(&src)];
    let mut files = vec![];

    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory).unwrap() {
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
