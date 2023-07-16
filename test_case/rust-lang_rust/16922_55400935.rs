 rust
use std::any::Any;

fn foo<T: Any>(value: &T) -> Box<Any> {
    box value as Box<Any>
}

fn main() {
    foo(&5i);
}
