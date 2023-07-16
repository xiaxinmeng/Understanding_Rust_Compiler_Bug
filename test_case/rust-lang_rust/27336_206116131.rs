 rust
#![feature(default_type_parameter_fallback)]
use std::path::Path;

fn func<P: AsRef<Path> = String>(p: Option<P>) {
    match p {
        None => { println!("None"); }
        Some(path) => { println!("{:?}", path.as_ref()); }
    }
}

fn main() {
    func(None);
}
