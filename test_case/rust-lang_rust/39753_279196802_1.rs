rust
use std::thread;

fn main() {
    thread::spawn(|| {
        loop {
            println!("123");
        }
    }).join();
}
