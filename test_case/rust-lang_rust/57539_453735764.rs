rust
#![feature(uniform_paths)]

mod foo {
    mod simple_command {
        use simple_command;
        use crate::foo::*;
    }
}

mod bar {}

fn main() {}
