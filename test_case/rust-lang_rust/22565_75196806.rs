 rust
trait B {
    extern "rust-call" fn b(i: i32) {}
}
struct A;
impl B for A {}
fn main () {
    <A as B>::b(10);
}
