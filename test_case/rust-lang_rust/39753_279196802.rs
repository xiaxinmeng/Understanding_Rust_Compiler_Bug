rust
use std::thread;

fn main() {
    thread::spawn(|| {
        loop {}
    }).join();
}
