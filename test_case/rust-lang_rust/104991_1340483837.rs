Rust
#[cfg(any(feature = "foo", feature = "bar", feature = "baz"))]
pub mod hi {
    // Something common to feature foo, bar and baz
    
    // Something only bar has
    #[cfg(feature = "bar")]
    pub fn bar_only () {}
}
