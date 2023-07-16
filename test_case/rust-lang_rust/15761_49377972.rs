 rust
extern crate time;

use std::io::IoResult;
use std::io::timer::Timer;
use std::sync::Future;

fn repro() -> IoResult<()> {
    let (tx, rx): (SyncSender<&'static str>, Receiver<&'static str>) = sync_channel(3);

    let mut f = Future::spawn(proc() {
        rx.recv();
        rx.recv();

        Ok(())
    });

    tx.try_send("zomg").unwrap();
    tx.try_send("hi2u").unwrap();

    f.get()
}

pub fn main() {
    let mut i = 0u;
    loop {
        i += 1;
        println!("starting iter {}; {}", i, time::precise_time_ns());
        repro().unwrap();
    }
}
