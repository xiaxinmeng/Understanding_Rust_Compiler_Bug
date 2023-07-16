 rust
extern crate time;

use std::io::IoResult;
use std::io::timer::Timer;
use std::sync::Future;

fn repro() -> IoResult<()> {
    let (tx1, rx1): (SyncSender<&'static str>, Receiver<&'static str>) = sync_channel(3);
    let (tx2, rx2): (SyncSender<&'static str>, Receiver<&'static str>) = sync_channel(3);

    spawn(proc() {
        rx1.recv();
        tx2.try_send("pong").unwrap();
    });

    tx1.try_send("ping").unwrap();
    rx2.recv();

    Ok(())
}

pub fn main() {
    let mut i = 0u;
    loop {
        i += 1;
        println!("starting iter {}; {}", i, time::precise_time_ns());
        repro().unwrap();
    }
}
