 rust
struct S;
trait T { }
impl AsRef<T+'static> for S {
    fn as_ref<'a>(&'a self) -> &'a (T+'a) {
        unimplemented!()
    }
}
fn main() {}
