rust
let mut new = 0;
ATOMIC.fetch_and_update(|x| {new = x + 1; new}, Ordering::Relaxed, Ordering::Relaxed);
