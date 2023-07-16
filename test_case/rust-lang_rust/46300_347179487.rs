
struct A;
impl From<A> for () {
    fn from(_: A) -> () {
        let f = || ();
        f()
    }
}
fn main() {
}
