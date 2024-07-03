// build.rs
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = "src";
    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(
        &dest_path,
        "// Generated content do not edit

pub fn message() -> &'static str {
    \"Hello, World!\"
}

pub fn init_all() {
    extension_a::init();
    extension_b::init();
}
"
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}