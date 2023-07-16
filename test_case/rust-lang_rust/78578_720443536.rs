rust
use core::cell::UnsafeCell;
struct NotAMutex<T>(UnsafeCell<T>);

unsafe impl<T> Sync for NotAMutex<T> {}

const FOO: NotAMutex<&i32> = NotAMutex(UnsafeCell::new(&42));
