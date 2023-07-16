rust
  let db: &RwLockHashSetStaticStr = get_str_db_rw_lock_ref();
  let (Ok(read_guard) | Err(read_guard)): Result<
    RwLockReadGuard<_>,
    RwLockReadGuard<_>,
  > = db.read().map_err(|e| e.into_inner());
