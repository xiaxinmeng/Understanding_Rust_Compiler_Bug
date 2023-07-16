rust
trait Foo {}

struct A;
impl Foo for A {}

struct B;
impl Foo for B {}

pub fn main() {
    let a = A;
    let b = B;

    // Works.
    let x: &dyn Foo = if true { &a } else { &b };

    // Doesn't work.
    let y = if true { &a } else { &b };
    let z: &dyn Foo = y;
}
