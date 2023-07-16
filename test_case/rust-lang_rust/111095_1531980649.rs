rust
#![crate_name = "foo"]

// @!has 'foo/hidden/index.html'
// @!has 'foo/hidden/inner/index.html'
// @!has 'foo/hidden/inner/trait.Foo.html'
#[doc(hidden)]
pub mod hidden {
    pub mod inner {
        pub trait Foo {
            /// Hello, world!
            fn test();
        }
    }
}

// @has 'foo/visible/index.html'
// @has 'foo/visible/trait.Foo.html'
#[doc(inline)]
pub use hidden::inner as visible;

// @has 'foo/struct.Bar.html'
// @count - '//*[@id="impl-Foo-for-Bar"]' 1
pub struct Bar;

impl visible::Foo for Bar {
    fn test() {}
}
