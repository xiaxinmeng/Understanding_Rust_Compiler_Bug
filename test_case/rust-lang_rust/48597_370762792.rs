rust
use std::iter::Extend;

struct MyType;
impl <'a, A> Extend<A> for &'a mut MyType {
    fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T) {
        // ...
    }
}
