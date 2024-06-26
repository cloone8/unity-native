use std::env;
use std::path::PathBuf;

use bindgen::Builder;

fn inject_bitfield_enums(builder: Builder) -> Builder {
    builder.bitfield_enum("UnityProfilerMarkerFlag_")
}

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        // .clang_arg("--std=c99")
        .clang_args(&["-x", "c++", "--std=c++17"])
        // The input header we would like to generate
        // bindings for.
        .header("unity/wrapper.h")
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let builder = inject_bitfield_enums(builder);

    // Finish the builder and generate the bindings.
    let bindings = builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
