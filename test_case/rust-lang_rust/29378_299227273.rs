rust
use std::thread;

fn spawn_and_join() -> thread::Result<()> {
    let child = thread::spawn(move || panic!("Oups"));
    child.join()
}

fn main() {
    match spawn_and_join() {
        Ok(_) => println!("this is fine"),
        Err(_) => println!("thread panicked"),
    }
}
