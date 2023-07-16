rust
#![feature(underscore_imports)]
#![allow(unused)]

macro_rules! m {
    ($item: item) => { $item $item }
}

m!(use std as _;); // ERROR the name `_` is defined multiple times

fn main() {}
