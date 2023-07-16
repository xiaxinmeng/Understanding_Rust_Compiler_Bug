rust
trait X {}
impl<T> X for T {}
fn foo<'a, T>(x: &'a u8, t: T) -> impl X + 'a {
    (x, t)
}
