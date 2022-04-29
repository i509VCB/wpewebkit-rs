use std::{path::PathBuf, env};

use bindgen::{Builder, CargoCallbacks};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper_egl.h");

    let bindings = Builder::default()
        .header("wrapper.h")
        // Only allow wpe types, functions and items
        .allowlist_type("wpe_.*")
        .allowlist_function("wpe_.*")
        .no_convert_floats()
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let egl_bindings = Builder::default()
        .header("wrapper_egl.h")
        // Exclude general wpe types since those are already generated.
        .blocklist_file("wrapper.h")
        // Only allow functions added in wpe_egl.h
        .allowlist_function("wpe_renderer_backend_egl_.*")
        // Block XKB types since those are already generated.
        .blocklist_item("xkb_.*")
        .no_convert_floats()
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    egl_bindings
        .write_to_file(out_path.join("egl.rs"))
        .expect("Couldn't write egl bindings!");
}
