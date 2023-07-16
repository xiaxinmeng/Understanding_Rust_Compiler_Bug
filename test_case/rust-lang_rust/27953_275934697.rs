rust
struct S;
trait T { }

impl AsRef<T> for S {
    fn as_ref(&self) -> &(T+'static) {
        unimplemented!()
    }
}
