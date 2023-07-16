 rust
use std::fs;

fn main() {
    fs::create_dir_all("./a/b/c").unwrap();
}
