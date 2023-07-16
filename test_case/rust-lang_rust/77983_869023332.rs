
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

pub const A: AtomicBool = AtomicBool::new(true);


fn main() {
    A.store(false, SeqCst);
    println!("Hello, world!");
}
