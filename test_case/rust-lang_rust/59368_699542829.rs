rust
pub mod foo {
    #[doc(hidden)]
    mod bar {
        pub struct Baz;
    }
}

#[doc(inline)]
pub use foo::bar;
