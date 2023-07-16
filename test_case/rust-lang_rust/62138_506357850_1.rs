rust
enum Foo {
    A,
    B,
    C(WithWraparoundInvalidValues),
}

fn main() {
    let x = Foo::B;
    if let Foo::C(_) = x {
        panic!();
    }
}
