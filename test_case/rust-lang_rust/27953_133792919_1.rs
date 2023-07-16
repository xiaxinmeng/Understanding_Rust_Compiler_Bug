 rust
struct S;
trait T { }
impl AsRef<T+'static> for S {
    fn as_ref(&self) -> &(T+'static) {
        unimplemented!()
    }
}
fn main() {}
