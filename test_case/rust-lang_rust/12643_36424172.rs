 rust
fn main() {
    'eggs: loop {
        'ham: for _ in range(0, 0) {
            if 1 + 1 == 2 {
                break 'eggs;
            } else {
                continue 'ham;
            }
        }
    }
}
