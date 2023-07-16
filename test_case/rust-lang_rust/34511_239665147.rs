 Rust
fn foo<'a: 'c,'b: 'c,'c>(x: &'a [u32], y: &'b [u32]) -> impl Iterator<Item=u32> + 'c {
    if condition { x.iter().cloned() } else { y.iter().cloned() }
}
