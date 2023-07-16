rust
pub fn foo() -> impl IntoIterator<IntoIter: Debug> {
    vec![5]
}
