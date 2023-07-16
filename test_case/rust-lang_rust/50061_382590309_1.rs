rust 
// bar.rs
#![feature(proc_macro)]

extern crate foo;

use foo::*;

macro_rules! j {
    (@ $v:tt) => {
        let _ = $v;
    };

    (# $v:tt) => {
        j!(@ $v);
    };
}

#[foo]
fn main() {
    //! test
    let s = "12'3";
    j!(# s);
}
