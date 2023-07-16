 rust
trait Foo<T> {}
struct Bar<T>;
impl Foo<int> for Bar<uint> {}
impl Foo<int> for Bar<int> {}

fn get<H: Foo<int>>(val: H) {}

fn main() {
    get(Bar);
}
