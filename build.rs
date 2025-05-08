use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=examples/");

    // Create necessary directories
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Create directories for generated code
    fs::create_dir_all(out_path.join("ir")).unwrap_or_else(|_| {
        println!("cargo:warning=Failed to create IR directory");
    });

    fs::create_dir_all(out_path.join("wasm")).unwrap_or_else(|_| {
        println!("cargo:warning=Failed to create WASM directory");
    });

    // TODO: Add code generation steps here when implemented
    println!("cargo:warning=Build script executed successfully");
} 