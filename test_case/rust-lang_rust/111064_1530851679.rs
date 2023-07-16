rust
#[doc(hidden)]
mod hidden {
    pub trait Foo {
        // Hello, world!
        fn test();
    }
}

pub use hidden::Foo;
