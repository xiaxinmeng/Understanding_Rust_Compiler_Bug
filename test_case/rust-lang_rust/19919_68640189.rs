 rust
#![feature(default_type_params)]

use std::thunk::Invoke;

fn foo(x: Box<for <'a>Invoke<&'a int, ()>>) {
}

fn bar() {
    foo( box () (move |: _: &int| () ))
}

fn main() {
}
