 rust
fn foo<'a>(v: &'a [int]) -> std::iter::Rev<std::vec::Items<'a, int>> {
    v.iter().rev()
}
