rust
#![deny(dead_code)]

enum X {
    A,
    B { y: () },
}
impl X {
    fn test(&mut self) {
        *self = Self::B { y: () };
    }
}
fn main() {
    let mut x = X::A;
    x.test();
}
