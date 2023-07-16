 rust

use std::thread::Thread;
use std::sync::mpsc::{channel};

fn main() {
    let (chs,chr) = channel();

    Thread::spawn(move || {
        chs.send(1u8); //must specifically annotate
    });
}
