 rust
use std::thread::spawn;

fn main() {
    let mut books = vec![1,2,3];
    spawn(|| books.push(4));
}
