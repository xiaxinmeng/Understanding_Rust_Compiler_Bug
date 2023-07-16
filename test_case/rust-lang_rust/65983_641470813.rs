rust
//crate a

pub struct Bar;

/// Link to [Bar]
pub struct Foo;

// crate b
pub use a::Foo;
