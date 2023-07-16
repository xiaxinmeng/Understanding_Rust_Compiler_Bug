 rust
struct S;
trait T { }
impl<'a> AsRef<T+'a> for S {
    fn as_ref<'b>(&'b self) -> &'b (T+'a) {
        unimplemented!()
    }
}
fn main() {
    S.as_ref();
}
