 rust
use std::sync::mpsc::Sender;

struct Foo {
    x: Sender<()>,
}

struct SomethingComplicated; /* lots of fields ... */

struct Bar {
    y: SomethingComplicated,
    z: Vec<Foo>,
    // etc.
}

fn test_sync<T: Sync>() {}

fn main() {
    test_sync::<Bar>();
}
