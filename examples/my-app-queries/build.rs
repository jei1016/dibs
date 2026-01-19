use dibs_query_gen::{generate_rust_code, parse_query_file};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=queries.styx");

    let queries_path = Path::new("queries.styx");
    let source = fs::read_to_string(queries_path).expect("Failed to read queries.styx");

    let file = parse_query_file(&source).expect("Failed to parse queries.styx");
    let generated = generate_rust_code(&file);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("queries.rs");

    fs::write(&dest_path, &generated.code).expect("Failed to write generated queries.rs");

    println!("cargo::rustc-env=QUERIES_PATH={}", dest_path.display());
}
