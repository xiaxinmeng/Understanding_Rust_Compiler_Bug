rs
#![feature(prelude_import)]
#![allow(unaligned_references)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate hdf5_derive;
#[repr(packed)]
struct P2(i8, u32);
#[allow(dead_code, unused_variables, unused_attributes)]
const _IMPL_H5TYPE_FOR_P2: () = {
    extern crate hdf5 as _h5;
    #[automatically_derived]
    unsafe impl _h5::types::H5Type for P2 {
        #[inline]
        fn type_descriptor() -> _h5::types::TypeDescriptor {
            let origin: *const P2 = ::std::ptr::null();
            let mut fields = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    _h5::types::CompoundField {
                        name: "0".to_owned(),
                        ty: <i8 as _h5::types::H5Type>::type_descriptor(),
                        offset: unsafe { &((*origin).0) as *const _ as _ },
                        index: 0,
                    },
                    _h5::types::CompoundField {
                        name: "1".to_owned(),
                        ty: <u32 as _h5::types::H5Type>::type_descriptor(),
                        offset: unsafe { &((*origin).1) as *const _ as _ },
                        index: 0,
                    },
                ]),
            );
            for i in 0..fields.len() {
                fields[i].index = i;
            }
            let size = ::std::mem::size_of::<P2>();
            _h5::types::TypeDescriptor::Compound(_h5::types::CompoundType {
                fields,
                size,
            })
        }
    }
};
#[automatically_derived]
impl ::core::marker::Copy for P2 {}
#[automatically_derived]
impl ::core::clone::Clone for P2 {
    #[inline]
    fn clone(&self) -> P2 {
        let _: ::core::clone::AssertParamIsClone<i8>;
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
