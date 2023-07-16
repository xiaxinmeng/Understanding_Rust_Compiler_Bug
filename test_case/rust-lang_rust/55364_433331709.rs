rust
/// See either [foo] or [bar].
pub mod sub {
    /// See [bar]
    pub fn foo() {}
    /// See [foo]
    pub fn bar() {}
}
