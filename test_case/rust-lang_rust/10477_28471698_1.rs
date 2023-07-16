 rust
    #[crate_type="lib"];
    pub use foo::Bar;
    mod foo {
        pub struct Bar; // not externally visible without the `pub use`
    }
    