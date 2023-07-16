 rust
use std::sync::Mutex;
use std::mem;

fn main() {
    let m = Mutex::new(4u);
    let mut g1 = m.lock();
    let mut g2 = m.lock();

    let p1 = &mut *g1;
    let p2 = &mut *g2;
    mem::swap(p1, p2);
}
