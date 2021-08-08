extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Tell cargo to tell rustc to link the libxml2 shared system library.
    println!("cargo:rustc-link-lib=xml2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/headers");

    let headers = fs::read_dir("src/headers").unwrap();
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
}
