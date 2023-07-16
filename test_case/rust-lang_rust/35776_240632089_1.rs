 rust
mod foo {
    use bar::f::*; // (1) Since this is not accessible in bar,
    // (4) ^ Thus we are able to resolve this import.
    pub fn f() {}
}
mod bar {
    pub use foo::f; // (2) we can deduce that this fails in the type namespace,
    pub use baz::*; // (3) so we know that `bar::f` will resolve to `baz::f` since it won't get shadowed.
}
mod baz {
    pub mod f {}
}
