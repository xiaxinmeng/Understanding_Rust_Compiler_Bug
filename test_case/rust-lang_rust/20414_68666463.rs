
trait Trait {
    fn method(self) -> int;
}
struct Wrapper<T> {
    field: T
}
impl<'a, T> Trait for &'a Wrapper<T> where &'a T: Trait {
    fn method(self) -> int {
        let r: &'a T = &self.field;
        r.method()
    }
}
fn main() {}
