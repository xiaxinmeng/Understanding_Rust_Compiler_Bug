
; file example.rs 
example.rs: empty
; rustc -Zunpretty=expanded example.rs
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
