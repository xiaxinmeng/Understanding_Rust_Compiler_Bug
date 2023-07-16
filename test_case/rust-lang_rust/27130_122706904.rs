 rust
use std::sync::Mutex;
use std::sync::Arc;

pub fn trim_in_place(a: &mut &[u8]) {
    while a.first() == Some(&42) {
        *a = &a[2..];
    }
}

fn main() {
    static X: &'static [u8] = &[42, 0, 42];
    let m = Arc::new(Mutex::new(X));
    let m2 = m.clone();
    let _ = std::thread::spawn(move || {
        let mut r = m.lock().unwrap();
        trim_in_place(&mut *r)
    }).join();
    let r = match m2.lock() {
        Ok(r) => r,
        Err(r) => r.into_inner()
    };
    assert_eq!(*r, &[42]);
}
