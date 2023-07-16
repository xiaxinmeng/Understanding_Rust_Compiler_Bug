Rust
fn counter() -> impl Iterator<Item=usize> {
    generate(Some(1), |it| {
        if it + 1 < 6 { Some(it + 1) } else { None }
    })
}
