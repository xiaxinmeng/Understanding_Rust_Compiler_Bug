rust
pub mod foo {
    #[doc(hidden)]
    mod bar {
        pub struct Baz;
    }
}

#[doc(inline, visible)]
pub use foo::bar;
