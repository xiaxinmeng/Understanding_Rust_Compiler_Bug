rust
#![feature(set_stdio, print_internals)]

use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::set_panic;

pub struct A;

impl Display for A {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        panic!();
   }
}

fn main() {
    set_panic(Some(Box::new(Vec::new())));
    std::io::_eprint(format_args!("{}", A));
}
