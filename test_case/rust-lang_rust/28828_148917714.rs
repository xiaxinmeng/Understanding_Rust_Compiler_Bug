 rust
trait Foo {
    type Out;
}

impl Foo for () {
    type Out = bool;
}

fn main() {
    type Bool = <() as Foo>::Out;
    let x: Bool = true; // Error E0308, mismatched types, expected <() as Foo>::Out
}
