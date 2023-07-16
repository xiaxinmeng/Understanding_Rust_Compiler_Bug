
example.rs:60:1: 64:2 error: conflicting implementations for trait `core::fmt::Debug` [E0119]
example.rs:60 impl<T> fmt::Debug for T where T: Foo {
example.rs:61     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
example.rs:62         write!(f, "{}", self.foo())
example.rs:63     }
example.rs:64 }
example.rs:60:1: 64:2 note: conflicting implementation in crate `core`
example.rs:60 impl<T> fmt::Debug for T where T: Foo {
example.rs:61     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
example.rs:62         write!(f, "{}", self.foo())
example.rs:63     }
example.rs:64 }
error: aborting due to 2 previous errors
