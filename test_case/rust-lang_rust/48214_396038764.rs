rust
#![feature(trivial_bounds)]

struct A;
trait X<T> {
    fn test(&self, t: T) {}
}

impl X<i32> for A {}

fn foo(a: A) where A: X<i64> {
    a.test(1i64); // expected i32, found i64
}
