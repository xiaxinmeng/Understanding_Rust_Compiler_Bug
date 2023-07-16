rust
trait Trait {
    type Type;
}

impl Trait for () {
    type Type = ();
}

type Bar = ();
fn foo() -> impl Trait<Type = Bar> {}

fn main() {
    foo();
}
