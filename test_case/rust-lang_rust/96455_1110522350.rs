rust
use std::fmt;

fn needs_late_drop() {
    struct Mutex;
    struct MutexGuard<'a>(&'a Mutex);

    impl<'a> MutexGuard<'a> {
        fn write_fmt(&self, _args: fmt::Arguments) -> &Self {
            self
        }

        fn lol(&self) {}
    }

    // NEEDS LATE DROP. Needs the MutexGuard temporary to drop at the nearest
    // semicolon _outside_ the write macro.
    let mutex = Mutex;
    write!(MutexGuard(&mutex), "").lol();
}

fn needs_early_drop() -> fmt::Result {
    struct Mutex;
    struct MutexGuard<'a>(&'a Mutex);

    impl<'a> Drop for MutexGuard<'a> {
        fn drop(&mut self) {}
    }

    impl<'a> MutexGuard<'a> {
        fn write_fmt(&self, _args: fmt::Arguments) -> fmt::Result {
            Ok(())
        }
    }

    // NEEDS EARLY DROP. Needs the MutexGuard temporary to drop at a semicolon
    // _inside_ of the write macro.
    let mutex = Mutex;
    write!(MutexGuard(&mutex), "") /* no semicolon */
}
