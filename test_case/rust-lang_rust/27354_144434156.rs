 rust
fn main() {
    let base = PathBuf::from("/some/path");
    let relative = PathBuf::from("./some/relative/path");
    let path = base.join(relative);
    println!("{:?}", path);
}
