
example.rs:60:1: 64:2 note: for a limited time, you can add `#![feature(old_orphan_check)]` to your crate to disable this rule
example.rs:60 impl<T> fmt::Debug for T where T: Foo {
example.rs:61     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
example.rs:62         write!(f, "{}", self.foo())
example.rs:63     }
example.rs:64 }
