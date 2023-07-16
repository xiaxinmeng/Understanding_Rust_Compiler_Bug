rust
fn counter() -> impl Iterator<Item=usize> {
    std::iter::iterate(0, move |count| {
        if count < 6 {
            Some((count + 1, count + 1))
        } else {
            None
        }
    })
}
