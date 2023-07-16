rust
fn inf() {
    match foo() {
        0 => inf(),
        1 => loop {},
        _ => inf(),
    }
}
