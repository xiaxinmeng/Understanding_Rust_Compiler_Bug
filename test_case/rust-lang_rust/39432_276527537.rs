
use std::thread;

fn worker() -> ! {
    panic!("woah");
}

fn main() {
    thread::spawn(worker).join().unwrap();
}
