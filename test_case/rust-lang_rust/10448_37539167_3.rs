 rust
fn foo<'a>(v: &'a [int]) -> <I:Iter> I {
    v.iter().rev()
}
