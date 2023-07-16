 rust
use std::thread;

fn main() {
    let r = thread::spawn(|| {
        panic!();
    }).join();
    println!("{:?}", r.is_err());
}
