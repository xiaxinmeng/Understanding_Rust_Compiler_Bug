 rust
fn main() {
    if true { 0 } else { 1 } + 2; // This is parsed as if there were a semicolon after the `}`
}
