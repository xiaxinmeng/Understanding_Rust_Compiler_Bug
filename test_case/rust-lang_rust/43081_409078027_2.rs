rust
#[macro_use]
extern crate proc_macro_bug;

#[derive(Testing)]
pub struct Bad {
    inner: usize,
}

pub fn main() { }
