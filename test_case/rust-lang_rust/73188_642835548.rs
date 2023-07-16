
use std::path::Path;

fn main() {
    let path = Path::new("/tmp/foo").join("bar.rs");
    println!("{}", path.display());
}
