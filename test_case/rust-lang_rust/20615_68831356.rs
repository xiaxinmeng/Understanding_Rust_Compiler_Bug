 rust
use std::thread::Thread;
use std::sync::mpsc::channel;
use std::rand::{self, Rng};
use std::io::timer::sleep;
use std::time::duration::Duration;

fn run_thread() {
    let (tx, rx) = channel();
    println!("running thread");
    let guard = Thread::spawn(move || {
        tx.send(rand::thread_rng().gen_range(0f32, 1f32)).unwrap();
        println!("thread sleeping");
        sleep(Duration::seconds(2));
        println!("thread finished sleeping");
    });
    if rx.recv().unwrap() < 0.5 {
        println!("detaching");
        guard.detach()
    } else {
        println!("joining")
    }
}

fn main() {
    run_thread();
    println!("exiting main thread");
}
