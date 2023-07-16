 rust
pub fn f() {}
pub mod foo {
    fn f() {}
}

mod bar {
    pub use f;
    pub use f as g;
    use foo::*; // This makes first import private but does not affect the second import.
}

mod baz {
    use f;
    pub use foo::*; // This makes the above import public
}
