 rust
pub use imp::foo;

mod imp {
    // this should require documentation even though impl::foo isn't a public path
    pub fn foo() {}
}
