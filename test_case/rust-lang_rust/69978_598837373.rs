rust
#![feature(rustc_private)]

extern crate rustc_data_structures;

use rustc_data_structures::owning_ref::{self, OwningRef};

fn main() {
    let buf = a.b();
    let buf: OwningRef<Vec<u8>, [u8]> = OwningRef::new(buf).into();
}
