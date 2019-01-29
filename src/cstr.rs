use crate::prelude::*;

pub struct CString {
  string: String,
}

impl CString {
  pub fn new(mut string: String) -> CString {
    string.push('\0');
    CString { string }
  }

  pub unsafe fn from_ptr(ptr: *const u8) -> CString {
    let mut string = String::new();
    let mut i = 0;
    loop {
      let cur = *ptr.add(i) as char;
      if cur == '\0' {
        break;
      }
      string.push(cur);
      i += 1;
    }
    CString::new(string)
  }

  pub fn as_ptr(&self) -> *const u8 {
    self.string.as_ptr()
  }

  pub fn into_string(self) -> String {
    self.string
  }
}

/// A replacement for str::to_string, as String::with_capacity errors for some reason
fn str_to_string(s: &str) -> String {
  let mut string = String::new();
  for c in s.chars() {
    string.push(c);
  }
  string
}

impl From<&str> for CString {
  fn from(s: &str) -> CString {
    CString::new(str_to_string(s))
  }
}

#[macro_export]
macro_rules! cstr {
  ($($var:ident),*) => {
    $(
      let $var = CString::from($var);
      let $var = $var.as_ptr();
    )*
  };
}

impl Into<String> for CString {
  fn into(self) -> String {
    self.into_string()
  }
}

pub unsafe fn ptr_to_string(ptr: *const u8) -> String {
  CString::from_ptr(ptr).into()
}
