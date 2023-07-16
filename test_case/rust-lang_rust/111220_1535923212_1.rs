rs
// crate b
pub trait B {
    fn f(&self);
}
impl B for b::A {
    fn f(&self) {
        let Self(a) = self;
        println!("{}", a);
    }
}
fn main() {
    let a = b::A::default();
    a.f();
}
