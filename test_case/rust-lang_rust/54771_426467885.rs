rust
trait Bar {}
impl Bar for u8 {}

fn bar<R: Bar>(_: impl Fn() -> R) {}
fn main() {
    bar(|| { 5u8; })
}
