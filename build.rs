extern crate bindgen;

extern crate cc;

#[path = "src/cli.rs"]
mod cli;

use cli::build_cli;

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{ crate_name, Shell };

//Relative to build.rs location
static C_DIRECTORY: &str = "src/c";
static HEADERS_DIRECTORY: &str = "src/headers";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Have gcc look for libraries in out_path
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());

    // Tell cargo to link the project's compiled c libs in out_path
    println!("cargo:rustc-link-lib=xmlparser");

    // Tell cargo to tell rustc to link the libxml2 shared system library.
    println!("cargo:rustc-link-lib=xml2");

    // Tell cargo to invalidate the built crate whenever the c code is changed
    println!("cargo:rerun-if-changed={}", C_DIRECTORY);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", HEADERS_DIRECTORY);

    let c_files: Vec<PathBuf> = fs::read_dir(C_DIRECTORY).unwrap()
        .into_iter().map(|f| f.unwrap().path()).collect();

    cc::Build::new()
        // Suppress unused parameter warnings
        .flag("-Wno-unused-parameter")
        // Suppress unused function warnings
        .flag("-Wno-unused-function")
        // Include path to header files
        .include(HEADERS_DIRECTORY)
        .files(c_files)
        .compile("xmlparser");

    let headers = fs::read_dir(HEADERS_DIRECTORY).unwrap();
    for header in headers {
        let mut header = header.unwrap().path();

        let bindings = bindgen::Builder::default()
            // Use types defined in core
            .use_core()
            // Use cty as the prefix of raw types
            .ctypes_prefix("cty")
            // Set include paths
            .clang_arg("-I/usr/include/libxml2")
            // The input header we would like to generate bindings for.
            .header(header.to_str().unwrap())
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        header.set_extension("rs");
        let header = header.file_name().unwrap();

        bindings.write_to_file(out_path.join(header)).unwrap();
    }

    let mut app = build_cli();
    app.gen_completions(crate_name!(), Shell::Zsh, &out_path);
    app.gen_completions(crate_name!(), Shell::Bash, &out_path);
    app.gen_completions(crate_name!(), Shell::Fish, &out_path);
}
