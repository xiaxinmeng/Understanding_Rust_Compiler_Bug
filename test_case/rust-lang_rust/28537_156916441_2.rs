 rust
mod foo {
    mod bar {
        /// MyType is _my_ type
        pub struct MyType;
    }

    pub use self::bar::MyType;
}

pub use self::foo::MyType;
