
use std::thread::Thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;

fn write_loop(rx: Receiver<uint>) {
    loop {
        match rx.try_recv() {
            Ok(v) => println!("v {}", v),
            _ => break
        }
    }
}

fn main() {
    let (tx, rx) = channel();
    Thread::scoped(move || {
        write_loop(rx);
    }).join();

    tx.send(3u);
}
