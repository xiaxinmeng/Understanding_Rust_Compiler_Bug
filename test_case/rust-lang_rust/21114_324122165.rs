Rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    if let Ok(_) = counter.lock() { }
}
