 rust
fn main() {
    match (0, 0) {
        (_, 0) => {}
        (0, ref _a) => {}
        _ => {}
    }
}
