rust
#![feature(extended_key_value_attributes)]

#[a = foo!()]
macro_rules! bar {
    () => {};
}

fn main() {}
