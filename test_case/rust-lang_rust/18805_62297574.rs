 rust
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate "std" as std;
extern crate "native" as rt;
#[prelude_import]
use std::prelude::*;
pub struct Wtf8Slice {
    bytes: [u8],
}
#[automatically_derived]
impl ::std::cmp::PartialEq for Wtf8Slice {
    #[inline]
    fn eq(&self, __arg_0: &Wtf8Slice) -> ::bool {
        match *__arg_0 {
            Wtf8Slice { bytes: ref __self_1_0 } =>
            match *self {
                Wtf8Slice { bytes: ref __self_0_0 } =>
                true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Wtf8Slice) -> ::bool {
        match *__arg_0 {
            Wtf8Slice { bytes: ref __self_1_0 } =>
            match *self {
                Wtf8Slice { bytes: ref __self_0_0 } =>
                false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
