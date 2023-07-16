rust
trait T {
    fn x() {}
}

impl<A> T for fn(A) {}
impl<A> T for fn(&A) {} // cause #[warn(coherence_leak_check)] on nightly 2020-03-29

fn main() {
    <fn(u8)>::x();
    <fn(&u8)>::x(); // requires the impl<A> T for fn(&A) to compile
}
