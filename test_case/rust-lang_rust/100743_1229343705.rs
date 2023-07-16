rust
use std::sync::atomic::AtomicU32;

#[repr(C, packed(4))]
struct FooT<T> {
    bar: T,
}

type Foo = FooT<AtomicU32>;

fn main() {
    let x: Foo = Foo { bar: AtomicU32::new(0) };
}
