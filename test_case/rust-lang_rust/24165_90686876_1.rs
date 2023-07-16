 rust
trait Foo<T> {}

impl<T> Foo<T> for T where T: Copy {}
impl<T> Foo<T> for Vec<T> where T: Copy {}

fn main() {
}
