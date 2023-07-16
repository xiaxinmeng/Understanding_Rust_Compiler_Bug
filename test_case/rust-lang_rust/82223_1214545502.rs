rust
let read_guard: RwLockReadGuard<_> =
    db.read().map_err(|e| e.into_inner()).into_ok_or_err();
