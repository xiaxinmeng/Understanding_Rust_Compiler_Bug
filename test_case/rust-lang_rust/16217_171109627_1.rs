 Rust
#[cfg(test)]
use std::thread;

#[test]
fn it_works() {
    thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
