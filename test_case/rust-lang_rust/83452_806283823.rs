rust
trait Trait {
    const F: usize = 99;
}

struct X;

impl Trait for X {}

const fn foo() -> usize {
    X::F
}
