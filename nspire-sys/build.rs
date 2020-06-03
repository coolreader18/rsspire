extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=ndls");
    println!("cargo:rustc-link-lib=syscalls");
    bindings();
}

fn bindings() {
    let bindings = base_bindings()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn base_bindings() -> bindgen::Builder {
    let output = Command::new("nspire-tools")
        .arg("path")
        .output()
        .expect("nspire-tools is not in path");
    assert!(output.status.success(), "nspire-tools failed");
    let ndless_path = std::str::from_utf8(&output.stdout)
        .expect("path was not utf-8")
        .trim()
        .trim_end_matches('/');
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("libc")
        .clang_args(
            [
                "lib/gcc/arm-none-eabi/*/include",
                "lib/gcc/arm-none-eabi/*/include-fixed",
                "include",
                "arm-none-eabi/include",
                "arm-none-eabi/sys-include",
            ]
            .iter()
            .map(|path| {
                let glob_path = format!("{}/{}", ndless_path, path);
                let path = glob::glob(&glob_path)
                    .unwrap()
                    .next()
                    .and_then(|res| res.ok())
                    .unwrap_or_else(|| panic!("no glob results for {}", glob_path));
                format!("-I{}", path.display())
            }),
        )
        .clang_arg("-nobuiltininc")
        .clang_arg("-nostdinc++")
        .clang_arg("-v")
        .clang_arg("-D _TINSPIRE")
}
