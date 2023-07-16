rust
use std::thread;
use std::sync::mpsc;

struct Handle {
    x: mpsc::Receiver<i32>,
    s: mpsc::Sender<i32>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        println!("{:?}", self.x.recv().unwrap());
        println!("{:?}", self.x.recv().unwrap());
    }
}

thread_local!(static HANDLE: Handle = new_handle());

fn new_handle() -> Handle {
    let (s, r) = mpsc::channel();
    s.send(10).unwrap();
    Handle { x: r, s: s, }
}

pub fn main() {
    let _s = thread::spawn(|| {
        HANDLE.with(|h| {
            h.s.send(15).unwrap();
        });
    });

    _s.join().unwrap();
}
