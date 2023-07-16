 rust
struct A;
#[derive(PartialEq)] struct B;
impl PartialEq<()> for B {
    fn eq(&self, _:&()) -> bool { false }
}
impl From<A> for B { fn from(_: A) -> B { B } } // trivial conversion

fn main() {
    B == A.into();
}
