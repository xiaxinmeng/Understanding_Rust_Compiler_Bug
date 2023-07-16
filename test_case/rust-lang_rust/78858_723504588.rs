rust
use std::path::Path;

fn main() {
    let path = Path::new("");

    println!("Empty string path");
    println!("{}", path.display());
    println!("Exists: {}", path.exists());
    match path.canonicalize() {
        Ok(p) => println!("{}", p.display()),
        Err(e) => println!("error {}", e),
    }
}
