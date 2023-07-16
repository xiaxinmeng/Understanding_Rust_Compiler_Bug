rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{current, spawn, park};

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    let thread_0 = current();
    let _thread_1 = spawn(move || {
        FLAG.store(true, Ordering::Relaxed);
        thread_0.unpark();
    });
    
    let thread_0 = current();
    let _thread_2 = spawn(move || {
        thread_0.unpark();
    });
    
    loop {
        park();
        if FLAG.load(Ordering::Relaxed) {
            return;
        }
    }
}
