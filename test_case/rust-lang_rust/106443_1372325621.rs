rust
struct S;
trait X {}
impl X for S {}
fn foo<T: X>(_: T) {}
fn bar<T: X + Clone>(s: &T) {
    foo(s.clone());
}
