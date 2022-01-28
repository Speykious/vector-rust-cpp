extern crate cpp_build;

use std::{env, path::Path};

fn main() {
    cpp_build::build("src/main.rs");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_name = "vector";
    let lib_path = Path::new(&dir).join(lib_name);
    let shared_lib_name = format!("lib{}.so", lib_name);
    let header_lib_name = format!("{}.hpp", lib_name);

    println!(
        "cargo:rerun-if-changed={}",
        lib_path.join(&header_lib_name).display(),
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    let input_path = lib_path.join(&shared_lib_name);
    let output_path = Path::new(&out_dir).join(&shared_lib_name);
    println!("cargo:warning=Copy: from {}", input_path.display());
    println!("cargo:warning=Copy: to {}", output_path.display());
    let res = std::fs::copy(input_path, output_path);
    println!("cargo:warning=Copy: {:?}", res);

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=vector");
}
