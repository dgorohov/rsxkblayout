extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::RustTarget;

fn main() {
    // Link X11 libraries
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=xkbfile");
    println!("cargo:rustc-link-lib=Xext");

    // Configure bindgen
    let config = bindgen::Builder::default()
        .rust_target(RustTarget::Stable_1_21)
        .header("wrapper.h");

    // Generate the bindings
    let bindings = config.generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
