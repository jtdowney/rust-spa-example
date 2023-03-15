use std::{env, fs, path::Path};

use quote::quote;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");

    let is_development = env::var_os("PROFILE")
        .map(|s| s == "debug")
        .unwrap_or_default();

    let code = quote! {
        pub const IS_DEVELOPMENT: bool = #is_development;
    };

    fs::write(dest_path, code.to_string()).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=db/migrations");
}
