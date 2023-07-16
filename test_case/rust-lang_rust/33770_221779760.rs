 rust
fn test_rwlock_rw() {
    let m = RwLock::new(0);
    let _g = m.read().unwrap();
    let _g2 = m.write().unwrap();
}
