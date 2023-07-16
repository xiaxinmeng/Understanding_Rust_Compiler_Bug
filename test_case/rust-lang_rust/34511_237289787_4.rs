 rust
fn foo<'a,'b>(x: &'a [u32], y: &'b [u32]) -> impl Iterator<Item=u32> {
    if condition { x.iter().cloned() } else { y.iter().cloned() }
}
