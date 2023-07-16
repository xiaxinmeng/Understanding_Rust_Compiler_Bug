rust
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel::<String>();

    thread::spawn(move || {
        let d = Duration::from_millis(10);
        loop {
            println!("recv");
            let _r = rx.recv_timeout(d);
        }
    });

    thread::sleep(Duration::from_millis(100));
    let _c1 = tx.clone();


    thread::sleep(Duration::from_secs(1));
}
