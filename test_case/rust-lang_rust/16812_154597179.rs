 rust
use std::any::Any;

enum MyOption<T: ?Sized> {
    None,
    Some(T)
}

fn main() {
    let _: Box<MyOption<Any>> = panic!();
}
