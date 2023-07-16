rust
use std::fs::OpenOptions;
fn main() {
    OpenOptions::new().write(false).truncate(true).open("test.txt").unwrap();
}
