
example.rs:60:1: 64:2 error: type parameter `T` is not constrained by any local type; only traits defined in the current crate can be implemented for a type parameter [E0210]
example.rs:60 impl<T> fmt::Debug for T where T: Foo {
example.rs:61     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
example.rs:62         write!(f, "{}", self.foo())
example.rs:63     }
example.rs:64 }
