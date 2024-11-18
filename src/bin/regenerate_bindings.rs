fn main () {
    if cfg!(feature = "generate-bindings") {
        use std::fs::copy;
        use std::path::PathBuf;
        
        copy(PathBuf::from(env!("OUT_DIR")).join("bindings.rs"), PathBuf::from("src/bindings.rs")).unwrap();
    } else {
        panic!("Must be run with 'generate-bindings' feature");
    }
}
