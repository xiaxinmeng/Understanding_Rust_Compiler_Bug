 rust
#![feature(unboxed_closures)]

use std::sync::Arc;

struct Foo {
    f: Arc<Box<Fn(&u8) + Send + Sync>>,
}

fn main() {
    let foo = Foo {
        f: Arc::new(Box::new(|_|{} as _))
    };
}
