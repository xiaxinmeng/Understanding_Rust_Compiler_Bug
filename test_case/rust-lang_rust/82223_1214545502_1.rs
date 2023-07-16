rust
let read_guard: RwLockReadGuard<_> =
    db.read().unwrap_or_else(PoisonError::into_inner);
