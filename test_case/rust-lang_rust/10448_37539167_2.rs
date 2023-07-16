 rust
fn foo<'a>(v: &'a [int]) -> Slice<'a, int>::Iter::Rev {
    v.iter().rev()
}
