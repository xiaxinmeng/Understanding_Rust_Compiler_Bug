rust
use std::pin::Pin;
use std::sync::Mutex;

pub struct Stderr(Mutex<()>);

pub fn poll_write(x: Pin<&mut Stderr>) { // `x` is not `mut` here
    let _ = &mut *x.0.lock().unwrap();
}
