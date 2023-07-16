 Rust
use std::num::Float;

trait Foo {
    type A: Float;

    // This does not work.
    fn sub_a(&self, a: <Self as Foo>::A, b: <Self as Foo>::A) -> <Self as Foo>::A {
        a - b
    }

    // However this does.
    fn sub_b(&self, a: <Self as Foo>::A, b: <Self as Foo>::A) -> <Self as Foo>::A {
        fn sub<T: Float>(a: T, b: T) -> T { a - b }
        sub(a, b)
    }
}

fn main() {}
