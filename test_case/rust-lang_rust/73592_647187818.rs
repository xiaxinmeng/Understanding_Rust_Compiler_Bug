rust
use std::pin::Pin;
use std::sync::Mutex;
use std::ops::Deref;

pub struct Stderr(Mutex<()>);

pub fn poll_write(x: Pin<&mut Stderr>) {
    let _ = &mut *Deref::deref(&x).0.lock().unwrap();
}
