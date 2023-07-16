 Rust
#[test]
fn it_works() {
    use std::thread;
    thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
