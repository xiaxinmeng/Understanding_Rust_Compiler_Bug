rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

use std::marker::PhantomData;

pub trait Precision {}

pub struct Satoshi;
impl Precision for Satoshi {}

type Inner = i64;

#[rustc_copy_clone_marker]
pub struct Amount<P: Precision = Satoshi>(Inner, PhantomData<P>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<P: ::std::marker::Copy + Precision> ::std::marker::Copy for Amount<P> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<P: ::std::clone::Clone + Precision> ::std::clone::Clone for Amount<P> {
    #[inline]
    fn clone(&self) -> Amount<P> {
        match *self {
            Amount(ref __self_0_0, ref __self_0_1) => Amount(
                ::std::clone::Clone::clone(&(*__self_0_0)),
                ::std::clone::Clone::clone(&(*__self_0_1)),
            ),
        }
    }
}
~\tmp\foo [master +14 ~0 -0 !]>
