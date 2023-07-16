rust
use std::ops::Index;

struct S;
struct X;
struct A;
struct B;

impl Index<S> for [X] {
    type Output = A;
    fn index(&self, _: S) -> &A {
        &A
    }
}

impl Index<S> for Vec<X> {
    type Output = B;
    fn index(&self, _: S) -> &B {
        &B
    }
}
