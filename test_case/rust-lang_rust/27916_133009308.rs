
use std::fs;
use std::iter;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    let long_string: String = iter::repeat("a").take(128).collect();
    for i in 0..10 {
        path.push(&long_string);
        println!("creating {}", i);
        fs::create_dir(&path).unwrap();
    }
    for i in (0..10).rev() {
        println!("removing {}", i);
        fs::remove_dir(&path).unwrap();
        path.pop();
    }
}
