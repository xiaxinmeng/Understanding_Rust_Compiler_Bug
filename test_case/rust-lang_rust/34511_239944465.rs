 rust
fn foo<'a, 'b>(x: &'a [u32], y: &'b [u32]) -> impl Iterator<Item=u32> {
    x.iter().chain(y).cloned()
}
