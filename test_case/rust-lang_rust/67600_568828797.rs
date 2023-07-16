rust
fn foo() {
    match Some((0, 1)) {
        Some((ref _x, _)) => {}
        _ => {}
    }
}
