rust
use std::{thread, io, mem, time::Duration};

fn main() {
    print!("hello");
    thread::spawn(|| mem::forget(io::stdout()));
    thread::sleep(Duration::from_secs(1));
}
