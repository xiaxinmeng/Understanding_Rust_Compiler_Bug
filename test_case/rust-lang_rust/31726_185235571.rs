 Rust
pub mod foo {
    use super::*;
    pub struct Bar { ... }
}
pub use foo::*;
// Plus a hundred more modules like this
