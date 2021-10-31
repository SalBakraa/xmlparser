#[path = "src/cli.rs"]
mod cli;

#[cfg(feature="pkgbuild")]
#[path = "build/pkgbuild.rs"]
mod pkgbuild;

use cli::build_cli;

#[cfg(feature="pkgbuild")]
use pkgbuild::write_pkgbuild;

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{ crate_name, Shell };

//Relative to build.rs location
static C_DIRECTORY: &str = "src/c";
static HEADERS_DIRECTORY: &str = "src/c/include";
static BUILD_MODULES: &str = "build";

fn main() {
    let out_path = env::var("OUT_DIR").unwrap();

    // Use 'target' as the target dir since there is no way to know if
    // the user changed it
    let target_dir = String::from("target");
    fs::create_dir_all(&target_dir).unwrap();

    // Write the out_path to file so that it can be used out of cargo
    fs::write(PathBuf::from(&target_dir).join("out_dir"), &out_path).unwrap();

    // Have gcc look for libraries in out_path
    println!("cargo:rustc-link-search={}", out_path);

    // Tell cargo to link the project's compiled c libs in out_path
    println!("cargo:rustc-link-lib=xmlparse");

    // Tell cargo to tell rustc to link the libxml2 shared system library.
    println!("cargo:rustc-link-lib=xml2");

    // Tell cargo to invalidate the built crate whenever the c code is changed
    println!("cargo:rerun-if-changed={}", C_DIRECTORY);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", HEADERS_DIRECTORY);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", BUILD_MODULES);

    let c_files: Vec<PathBuf> = fs::read_dir(C_DIRECTORY).unwrap()
        .map(|e| e.unwrap().path())
        .filter(|e| e.as_path().as_os_str() != HEADERS_DIRECTORY)
        .collect();

    cc::Build::new()
        // Suppress unused parameter warnings
        .flag("-Wno-unused-parameter")
        // Suppress unused function warnings
        .flag("-Wno-unused-function")
        // Include path to header files
        .flag("-iquote").flag(HEADERS_DIRECTORY)
        .files(c_files)
        .compile("xmlparse");

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

        bindings.write_to_file(PathBuf::from(&out_path).join(header)).unwrap();
    }

    let mut app = build_cli();
    app.gen_completions(crate_name!(), Shell::Zsh, &out_path);
    app.gen_completions(crate_name!(), Shell::Bash, &out_path);
    app.gen_completions(crate_name!(), Shell::Fish, &out_path);

    #[cfg(feature="pkgbuild")]
    write_pkgbuild().unwrap();
}
