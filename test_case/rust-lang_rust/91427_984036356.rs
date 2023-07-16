rust
#[macro_export]
macro_rules! foo {
    () => {};
}


pub mod bar {
    /// [foo!]
    pub fn baz() {
        foo!();
    }
}
