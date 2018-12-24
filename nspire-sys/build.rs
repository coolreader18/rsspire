extern crate bindgen;

use std::env;
use std::path::PathBuf;

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
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

fn base_bindings() -> bindgen::Builder {
  bindgen::Builder::default()
    .use_core()
    .ctypes_prefix("libc")
    .clang_arg(
      "-I/usr/share/ndless/ndless-sdk/toolchain/install/lib/gcc/arm-none-eabi/8.2.0/include",
    )
    .clang_arg(
      "-I/usr/share/ndless/ndless-sdk/toolchain/install/lib/gcc/arm-none-eabi/8.2.0/include-fixed",
    )
    .clang_arg("-I/usr/share/ndless/ndless-sdk/include")
    .clang_arg("-I/usr/share/ndless/ndless-sdk/toolchain/install/arm-none-eabi/include/")
    .clang_arg(
      "-I/usr/share/ndless/ndless-sdk/toolchain/install/lib/gcc/arm-none-eabi/8.2.0/include/",
    )
    .clang_arg("-I/usr/share/ndless/ndless-sdk/toolchain/install/arm-none-eabi/sys-include/")
    .clang_arg("-nobuiltininc")
    .clang_arg("-nostdinc++")
    .clang_arg("-v")
    .clang_arg("-D _TINSPIRE")
}
