#[cfg(feature = "generate-bindings")]
extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib=ldap");
    println!("cargo:rustc-link-lib=sasl2");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "generate-bindings")]
    generate_bindings();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    use std::env;
    use std::path::PathBuf;

    let clang_include_args = vec![
        "-I/usr/include".to_string(),
    ];

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .clang_args(clang_include_args)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
