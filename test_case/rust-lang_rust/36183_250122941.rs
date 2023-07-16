 rust
use std::any::Any;

fn foo<T: Any>(value: &T) -> Box<Any> {
    Box::new(value) as Box<Any>
}

fn main() {
    foo(&5);
}
