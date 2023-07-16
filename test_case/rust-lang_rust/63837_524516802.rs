rust
fn main() {
    'outer: loop {
        || break 'outer;
    }
}
