rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

fn main() {

    // This is a comment !!!!
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello world\n"],
                &[]));
    };
}
