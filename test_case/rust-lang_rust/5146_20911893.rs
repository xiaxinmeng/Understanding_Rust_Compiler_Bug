
use std::sys;
struct Foo {
        bar: [Option<Foo>, ..2]
}
fn main() {
    error!("%?", sys::size_of::<Foo>());
}

