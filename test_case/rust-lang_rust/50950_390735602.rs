rust
struct X;
impl X {
    fn g(&self, _: impl Send) {}
}
fn main() {
    X.g::<u64>(0);
}
