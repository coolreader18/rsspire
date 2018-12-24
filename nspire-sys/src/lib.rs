#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![no_std]

pub mod libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
