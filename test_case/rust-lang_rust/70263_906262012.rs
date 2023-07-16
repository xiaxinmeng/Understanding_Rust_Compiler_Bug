rust
impl<'a, R: RawMutex + 'a, T: ?Sized + 'a> MutexGuard<'a, R, T> {
    // ..

    pub fn map<U, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>
    where
        F: FnOnce(&'a mut T) -> U,
    {
        // ..
    }

    // ..
}

#[test]
fn test_map() {
    use crate::{lock_api::RawMutex, Mutex, MutexGuard};

    const FOO: usize = 0;

    static OUTER: Mutex<&usize> = Mutex::const_new(RawMutex::INIT, &FOO);
    static M: Mutex<usize> = Mutex::const_new(RawMutex::INIT, 0);

    let guard = MutexGuard::map(M.lock(), |inner: &'static mut usize| {
        *OUTER.lock() = inner;
    });
    drop(guard);

    let outer: &usize = &*OUTER.lock();

    assert_eq!(*outer, 0);

    *M.lock() = 1;

    assert_eq!(*outer, 1);
}
