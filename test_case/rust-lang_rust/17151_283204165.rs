
use std::ops::Deref;
struct S;
struct T;
static USIZE: usize = 5;
impl Deref for T {
    type Target = usize;
    fn deref(&self) -> &Self::Target { &USIZE }
}
trait Check<T> {
    fn check(&self, _: &T) {}
}
impl Check<usize> for S {}
//impl Check<u64> for S {} // XXX
fn main() {
    S.check(&T);
}
