
pub mod a {
    /// A foo.
    pub struct Foo;
}

pub mod b {
    /// This doesn't show up anywhere
    pub use a::Foo as Bar;
}
