rust
struct A;

impl Rem for A {
    fn rem(self, rhs: Self) -> Self::Output {
        A // expected type `<A as std::ops::Rem>::Output`
    }
}
