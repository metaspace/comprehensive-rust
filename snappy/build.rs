use std::error::Error;
use tap::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let snappy = pkg_config::Config::new()
        .atleast_version("1.2.1")
        .probe("snappy")
        .unwrap();

    let flags: Vec<_> = snappy
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.to_string_lossy()))
        .collect();

    let header = snappy
        .include_paths
        .iter()
        .map(|p| (*p).to_owned().tap_mut(|p| p.push("snappy-c.h")))
        .find(|p| p.is_file())
        .unwrap();

    // Generate bindings for snappy
    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .allowlist_function("snappy_.*")
        .allowlist_type("snappy_.*")
        .clang_args(flags)
        .generate()
        .expect("Failed to generate bindings");

    // Write bindings to file
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings to file");

    Ok(())
}
