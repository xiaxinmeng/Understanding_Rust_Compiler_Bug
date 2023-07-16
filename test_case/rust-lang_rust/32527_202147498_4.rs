 rust
impl<'rwlock, T: 'rwlock> RwLockWriteGuard<'rwlock, T> {
    pub fn downgrade(self) -> LockResult<RwLockReadGuard<'rwlock, T>> {
        poison::map_result(self.poison.borrow(), |_| {
            self.__lock.lock.downgrade();
            RwLockReadGuard {
                __lock: self.__lock,
                __data: &*self.__data,
            }
        }
    }
}
