 rust
mod foo {
    pub mod quux {}
}
mod bar {
    pub use foo::*;
    pub use baz::quux::*; // We need to resolve this to deduce that `bar::quux` isn't ambiguous,
}
mod baz {
    pub use bar::*; // but that requires importing `quux` here, which we wouldn't be able to do until we have deduced that `quux` isn't ambiguous.
}
