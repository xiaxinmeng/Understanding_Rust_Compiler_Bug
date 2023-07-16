rust
struct A {}
struct B {}

impl From<A> for B {}

fn main() {
    let c1 = A {};
    c1::<Into<B>>;
}
