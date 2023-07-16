rust
#![deny(clippy::use_self)]
#![deny(dead_code)]

#[derive(Debug)]
enum X {
    A { _a: () },
    B { _b: () },
}
impl X {
    fn a() -> X {
        X::A { _a: () }
    }
    fn b() -> Self {
        Self::B { _b: () }
    }
}
pub fn main() {
    println!("{:?}", X::a());
    println!("{:?}", X::b());
}
