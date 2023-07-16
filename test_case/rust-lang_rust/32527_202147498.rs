 rust
impl<'rwlock, T: 'rwlock> RwLockWriteGuard<'rwlock, T> {
    pub fn downgrade(self) -> LockResult<RwLockReadGuard<'rwlock, T>> {
        // maps to pthread_rwlock_rdlock(), which is supposed to succeed unconditionally.
        poison::map_result(self.poison.borrow(), |_| {
            self.__lock.lock.read();
            RwLockReadGuard {
                __lock: self.__lock,
                __data: &*self.__data,
            }
        }

        // write guard dropped here, maps to pthread_rwlock_unlock()
        // which is supposed to release the write lock first.
    }
}
