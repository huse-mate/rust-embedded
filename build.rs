use std::env::{self, current_dir};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use bindgen;
use cc;

fn main() {

// This build script compiles all the C libraries included in the bind_path 
// location's .c file and generates a rust file which includes all the 
// declerations and links them to the definitions in the compiled c library

    let bind_path = "src/bindings.c";

    cc::Build::new()
        .file(bind_path)
        .compile("clib");

 
    let bindings = bindgen::Builder::default().ctypes_prefix("cty").use_core()
        .header(bind_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate().expect("couldn't generate binder");

    
    let out_path = current_dir().unwrap();
    bindings.write_to_file(out_path.join("src/bindings.rs")).expect("couldn't generate binding rs file");
   



//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.


    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
    
    
}
