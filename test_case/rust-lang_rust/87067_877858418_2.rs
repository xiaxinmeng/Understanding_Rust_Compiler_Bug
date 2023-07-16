rs
// still in crate `b`
use a::A;
impl PartialEq<A> for B {
    pub fn eq(&self, _other: &A) -> bool {
        true
    }
}
impl PartialEq<B> for A {
    pub fn eq(&self, _other: &B) -> bool {
        true
    }
}
