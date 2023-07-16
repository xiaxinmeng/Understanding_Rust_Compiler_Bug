rust
use std::{
    io::ErrorKind,
    mem::forget,
    thread::{sleep, Builder},
    time::Duration,
};

fn idle() {
    loop {
        sleep(Duration::from_secs(60));
    }
}

fn main() {
    let mut threads = 0;
    let mut stuck = false;
    loop {
        match Builder::new().stack_size(4096).spawn(idle) {
            Ok(handle) => {
                threads += 1;
                if stuck {
                    println!("progress after {}", threads);
                }
                stuck = false;
                forget(handle);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                if !stuck {
                    println!("wouldblock after {}", threads);
                }
                stuck = true;
                sleep(Duration::from_millis(100));
            }
            Err(e) => {
                panic!("{}: {:?}", threads, e.kind());
            }
        }
    }
}
