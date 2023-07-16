 rust
fn foo<'a>(v: &'a [int]) -> std::vec::Slice<'a, int>::Iter::Rev {
    v.iter().rev()
}
