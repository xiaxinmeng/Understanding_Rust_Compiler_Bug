rust
enum A {
    A1,
    A2,
}

enum B {
    B1,
    B2,
}

impl PartialEq<B> for A {
    fn eq(&self, other: &B) -> bool {
        match (self, other) {
            (A::A1, B::B1) => true,
            (A::A1, B::B2) => true,
            (A::A2, B::B1) => true,
            (A::A2, B::B2) => false,
        }
    }
}
