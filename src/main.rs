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
// extern crate cstr_core;
// extern crate memchr;

extern crate nspire_sys as sys;
mod global_allocator;

use core::panic::PanicInfo;
pub use sys::libc;

pub mod prelude {
  pub use crate::ToCChar;
  pub use alloc::prelude::*;
  pub use alloc::str::FromStr;
  // pub use cstr_core::CStr;
}
use self::prelude::*;
use alloc::str;

#[global_allocator]
static GLOBAL: global_allocator::NspireAlloc = global_allocator::NspireAlloc;

pub trait ToCChar {
  fn to_c_char(&self) -> *mut libc::c_char;
}

impl ToCChar for str {
  fn to_c_char(&self) -> *mut libc::c_char {
    let mut bytes = self.to_string().into_bytes();
    bytes.push(b'\0');
    bytes
      .into_iter()
      .map(|b| b as libc::c_char)
      .collect::<Vec<_>>()
      .as_mut_ptr()
  }
}
impl ToCChar for String {
  fn to_c_char(&self) -> *mut libc::c_char {
    let mut bytes = self.clone().into_bytes();
    bytes.push(b'\0');
    bytes
      .into_iter()
      .map(|b| b as libc::c_char)
      .collect::<Vec<_>>()
      .as_mut_ptr()
  }
}

// impl From<String> for *mut libc::c_char {}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut c = 0;
  let mut vec = vec![];
  let mut string = String::new();
  loop {
    let mut ptr = core::ptr::null_mut::<libc::c_char>();
    unsafe {
      // sys::exit(1);
      sys::show_msg_user_input(
        b"a\0".as_ptr() as *const i8,
        b"b\0".as_ptr() as *const i8,
        b"c\0".as_ptr() as *const i8,
        &mut ptr,
      );
    }
    vec.push(ptr);
    let mut i = 5;
    loop {
      unsafe {
        let c = *ptr.offset(i) as u8;
        if c == b'\0' {
          break;
        }
        string.push(c as char);
      }
      i += 1;
    }
    c += 1;
    if c == 5 {
      break;
    }
  }
  for a in vec {
    unsafe {
      sys::_show_msgbox(
        b"hey\0".as_ptr() as *const i8,
        a,
        1,
        b"k\0".as_ptr() as *const i8,
      );
    }
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
