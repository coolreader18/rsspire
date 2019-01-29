#![feature(
  alloc,
  lang_items,
  core_intrinsics,
  alloc_error_handler,
  never_type,
  start
)]
#![no_std]

#[macro_use]
extern crate alloc;

extern crate nspire_sys as sys;
mod cstr;
mod global_allocator;

use core::panic::PanicInfo;
pub use sys::libc;

pub mod prelude {
  pub use crate::cstr::*;
  pub use alloc::prelude::*;
  pub use alloc::str::FromStr;
}
use self::prelude::*;
use alloc::str;

#[global_allocator]
static GLOBAL: global_allocator::NspireAlloc = global_allocator::NspireAlloc;

fn show_msg_user_input(title: &str, msg: &str, default: &str) -> String {
  cstr!(title, msg, default);
  let mut ptr = core::ptr::null_mut();
  unsafe {
    sys::show_msg_user_input(title, msg, default, &mut ptr);
    ptr_to_string(ptr)
  }
}

fn show_msgbox(title: &str, msg: &str, button: &str) {
  cstr!(title, msg, button);

  unsafe {
    sys::_show_msgbox(title, msg, 1, button);
  }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut vec = Vec::new();
  for c in 1..5 {
    let response = show_msg_user_input("hey", &format!("number {}", c), "ya");

    vec.push(response);
  }
  for a in vec {
    show_msgbox("look what you did", &a, "k");
  }
  0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {}

#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_info: &PanicInfo) -> ! {
  unsafe { core::intrinsics::abort() }
}

// #[lang = "oom"]
// extern "C" fn foo(_: core::alloc::Layout) -> ! {
//   // example implementation based on libc
//   extern "C" {
//     fn exit(code: libc::c_int) -> !;
//   }
//   unsafe { exit(1) }
// }

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
  extern "C" {
    fn exit(code: libc::c_int) -> !;
  }
  unsafe { exit(1) }
}
