
$ rustc hello.rs --pretty=expanded -Z unstable-options
#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
pub enum Foo { A1, A2, A3, }
#[automatically_derived]
impl ::std::cmp::Eq for Foo {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        match (&*self,) {
            (&Foo::A1,) => { }
            (&Foo::A2,) => { }
            (&Foo::A3,) => { }
        }
    }
}
#[automatically_derived]
impl ::std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, __arg_0: &Foo) -> bool {
        match (&*self, &*__arg_0) {
            (&Foo::A1, &Foo::A1) => true,
            (&Foo::A2, &Foo::A2) => true,
            (&Foo::A3, &Foo::A3) => true,
            _ => {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as i32;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as i32;
                false
            }
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Foo) -> bool {
        match (&*self, &*__arg_0) {
            (&Foo::A1, &Foo::A1) => false,
            (&Foo::A2, &Foo::A2) => false,
            (&Foo::A3, &Foo::A3) => false,
            _ => {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as i32;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as i32;
                true
            }
        }
    }
}

fn main() { }
