rust
trait Foo {}
trait Bar {}

struct A;
impl Foo for A {}
impl Bar for A {}

struct B;
impl Foo for B {}
impl Bar for B {}

fn main() {
    // Should the compiler infer `x` to be of type `Box<Foo>` or `Box<Bar>`?
    let x = match true {
        true => Box::new(A),
        false => Box::new(B),
    };
    drop(x);
}
