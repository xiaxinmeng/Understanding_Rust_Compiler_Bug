
use std::pin::Pin;
use std::sync::Mutex;

pub struct Stderr(Mutex<()>);

pub fn poll_write(mut x: Pin<&mut Stderr>) {
    let _ = &mut *x.0.lock().unwrap();
}
