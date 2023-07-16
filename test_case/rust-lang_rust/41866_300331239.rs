rust
use std::path::{Component, PathBuf, Path};

fn main() {
    let path = Path::new("a/b/c/d");
    println!("{:?}", path.components().map(Component::as_os_str).filter(|c| *c != "b").collect::<PathBuf>());
}
