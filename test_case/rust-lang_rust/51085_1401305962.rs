rust
fn never(a: std::convert::Infallible) -> std::convert::Infallible {
    match a {}
}

fn never_never(
    a: std::convert::Infallible,
    b: std::convert::Infallible,
) -> std::convert::Infallible {
    match (a, b) {}
}
