 rust
#[cfg(tests)]
pub mod tests {
    use std::thread;

    #[test]
    fn it_works() {
        thread::spawn(|| {
            println!("Hello from a thread!");
        });
    }
}
