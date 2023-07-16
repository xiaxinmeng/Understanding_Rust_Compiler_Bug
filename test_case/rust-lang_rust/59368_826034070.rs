rust
pub mod foo {
    /// `macro_rules!`
    #[macro_export]
    macro_rules! m { () => () }

    /// `pub use m;`
    #[doc(inline)]
    pub use m;
}
