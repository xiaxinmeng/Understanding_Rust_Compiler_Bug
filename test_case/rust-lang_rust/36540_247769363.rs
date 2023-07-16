 rust
macro_rules! mac {
    () => { 0 }
}

trait Foo {
    type Assoc;
}

impl Foo for [(); mac!()] {
    type Assoc = [(); mac!()];
}

fn main() {}
