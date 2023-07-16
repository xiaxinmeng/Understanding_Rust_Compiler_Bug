rust
// in bar.rs
struct Foo(String);

// in lib.rs
mod bar;
mod maybe_type {
    #[cfg(feature = "feat-a")]
    mod inner {
        pub type Maybe = String;
    }

    #[cfg(not(feature = "feat-a"))]
    mod inner {
        pub type Maybe = crate::bar::Foo;
    }

    pub use inner::Maybe;
}
