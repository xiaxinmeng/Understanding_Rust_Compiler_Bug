rust
use std::thread;

fn main() {
    let child = thread::spawn(move || panic!("Oups"));

    let res: thread::Result<()> = child.join();

    match res {
        Ok(_) => println!("this is fine"),
        Err(_) => println!("thread panicked"),
    }
}
