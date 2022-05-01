use std::path::Path;
use std::{env, fs};

pub fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let gen_path = Path::new(out_dir).join("identifiers.in");
    env::set_var("DERIVE_OUT_DIR", &gen_path);

    println!("cargo:rerun-if-changed=build.rs");

    if !gen_path.exists() {
        fs::write(gen_path, "&[]").unwrap();
    }
}
