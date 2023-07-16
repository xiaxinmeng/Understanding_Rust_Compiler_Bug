rust
let _x = AtomicU128::new(3).fetch_max(7, Relaxed);
