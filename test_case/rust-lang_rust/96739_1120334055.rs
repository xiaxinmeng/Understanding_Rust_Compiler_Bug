rs
use std::env::temp_dir;
use std::fs::File;
fn main() {
    let file = File::create(temp_dir.as_path().join("lolz")).unwrap();
}
