 rust
fn main() {
    match (0, 0) {
        (0, ref _a) => {}
        (_, 0) => {}
        _ => {}
    }
}
