rust
use std::thread;
use std::fs;

fn copy_in_thread() -> thread::Result<()> {
    thread::spawn(move || { fs::copy("foo.txt", "bar.txt").unwrap(); }).join()
}

fn main() {
    match copy_in_thread() {
        Ok(_) => println!("this is fine"),
        Err(_) => println!("thread panicked"),
    }
}
