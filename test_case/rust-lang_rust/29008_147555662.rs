
use std::path::PathBuf;
fn main() {
    let mut p4 = PathBuf::from("/a/b/c/..");
    println!("{:?} {:?}", p4, p4.file_name());
    println!("pop: {}", p4.pop());
    println!("{:?} {:?}", p4, p4.file_name());
}
