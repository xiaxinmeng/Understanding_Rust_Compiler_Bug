 rust
use std::mem;
enum Foo {
    Bar(u8, u32),
    Baz
}

fn main() {
    println!("{} {}", mem::size_of::<Foo>(), mem::align_of::<Foo>())
}
