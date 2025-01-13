use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("RUN_BINDGEN").is_err() {
        println!("Skipping bindgen; set RUN_BINDGEN=1 to enable.");
        return;
    }
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./extern/includes/libil2cpp/il2cpp/libil2cpp")
        .clang_arg("-v")
        .wrap_unsafe_ops(true)
        .sort_semantically(true) // Incluye las cabeceras si es necesario
        .generate()
        .expect("Unable to generate bindings");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let output_path = current_dir.join("bindings_out.rs");

    bindings
        .write_to_file(&output_path)
        .expect("Couldn't write bindings!");
    println!("Returning early");
    panic!("Written bindings to {}", output_path.display());
}
