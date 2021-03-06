extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    bindgen::builder()
        .header("./include/emacs-module.h")
        .whitelist_type("^emacs.*")
        .whitelist_function("^emacs.*")
        .whitelist_var("^emacs.*")
        .prepend_enum_name(false)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("emacs_module.rs"))
        .unwrap();
}
