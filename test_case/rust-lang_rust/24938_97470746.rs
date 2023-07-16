 rust
// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(associated_consts)]

trait AsFixedSizeArray: Sized {
    const SIZE: usize;
    unsafe fn as_fixed_size_array(&self)
                                  -> &[u8; <Self as AsFixedSizeArray>::SIZE];
}

impl AsFixedSizeArray for u8 {
    const SIZE: usize = 1;
    fn as_fixed_size_array(&self)
                           -> &[u8; <Self as AsFixedSizeArray>::SIZE] {
        &[*self]
    }
}

fn main() {
    assert_eq!([1u8], *(1u8).as_fixed_size_array());
}
