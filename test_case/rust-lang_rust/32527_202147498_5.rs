 rust
/// Downgrades the write lock to a read lock without allowing any other 
/// writers to grab the lock, only blocking if necessary.
///
/// On Windows systems, the underlying SRWLock does not support
/// non-blocking downgrades.
/// On Unix systems, this should return instantly.
pub fn downgrade(self) -> LockResult<RwLockReadGuard<'rwlock, T>> { ... }
