rust
use std::path::PathBuf;
fn main() {
    let mut p = PathBuf::from("test.txt.exe");
    p.set_extension("");
    println!("{}", p.display());
}
