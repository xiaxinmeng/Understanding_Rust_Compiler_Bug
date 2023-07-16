rust
fn blah(a: &[i32; 2]) -> Option<&[i32]> {
    true.then(|| { a })
}
