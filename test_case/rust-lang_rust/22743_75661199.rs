 rust
use std::ops::Mul;

pub struct Foo {
    x: f64,
}

impl Mul<Foo> for f64 {
    type Output = Foo;

    fn mul(self, rhs: Foo) -> Foo {
        println!("Multiplying!");
        rhs
    }
}

pub fn main() {
    let f: Foo = Foo { x: 5.0 };
    let val: f64 = 3.0;
    let f2: Foo = Mul::mul(val, f);
}
