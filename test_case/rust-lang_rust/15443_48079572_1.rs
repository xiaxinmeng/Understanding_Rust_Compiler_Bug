 rust
use A::B;

mod A {
    pub struct B { n: int }
    fn B() { }
}

fn main() {
    let _ = B();
}
