 rust
let lock = RwLock::new();
mem::forget(lock.read());
lock.read();
