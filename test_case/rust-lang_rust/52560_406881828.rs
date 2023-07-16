rust
use std::fmt;
impl<B: Bar<Item=Bi>, Bi: fmt::Debug> fmt::Debug for Foo<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Foo").field(&self.0).finish()
    }
}
