 rust
mod foo {
    pub fn bar() {}
    pub mod bar {}
    pub use self::bar as baz; // Now this resolves
    pub use self::baz::*;
}
