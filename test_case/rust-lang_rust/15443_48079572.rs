 rust
use A::B;

mod A {
    pub mod B {
        pub struct C { pub n: int }
    }
    pub struct B { pub n: int }
}

fn main() {
    let _ = B { n: 1 };
    let _ = B::C { n: 1 };
}
