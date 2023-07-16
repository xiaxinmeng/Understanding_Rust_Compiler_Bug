 rust
// foo.rs
pub use foo::*;

mod foo {
    pub fn foo() { println("foo"); }
}
