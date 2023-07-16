
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct Barrier2 {
    c: Arc<AtomicUsize>,
}

impl Barrier2 {
    fn new() -> (Barrier2, Barrier2) {
        let a = Arc::new(AtomicUsize::new(0));
        (Barrier2 { c: a.clone() }, Barrier2 { c: a })
    }

    fn wait(&self) {
        self.c.fetch_add(1, Ordering::SeqCst);
        while self.c.load(Ordering::SeqCst) != 2 {
        }
    }
}

fn main() {
    for a in 0.. {
        println!("iter {}", a);

        let (a, b) = Barrier2::new();

        let (tx, rx) = channel();
        
        let th = thread::spawn(move || {
            a.wait();
            loop {
                match rx.recv_timeout(Duration::from_millis(1)) {
                    Ok(_) => {
                        break;
                    },
                    Err(_) => {
                    },
                }
            }
        });

        b.wait();
        thread::sleep(Duration::from_millis(1));
        tx.clone().send(()).expect("send");
        th.join().unwrap();
    }
}
