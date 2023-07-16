rust
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

fn main() {
    let (send, recv) = channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let send = send.clone();
        thread::sleep(Duration::from_secs(1));
    });

    loop {
        match recv.recv_timeout(Duration::from_millis(20)) {
            Ok(s) => {let r: String = s;}
            Err(_) => {}
        }
    }
}
