 rust
mod foo {
    /// MyType is _my_ type
    pub struct MyType;
}

pub use self::foo::MyType;
