Rust
// Just like the std::sync:Mutex.
struct Mutex<T> {
  sys_mutex: ...,
  cell: UnsafeCell<T>
}

impl<T> Mutex<T> {
  pub fn lock(self) -> Locked<T> {
    // do actual low-level OS mutex lock
    self.sys_mutex.lock();
    return Locked { locked_mutex: self };
  }
}

struct Locked<T> {
  locked_mutex: Mutex<T>
}

impl<T> Locked<T> {
  pub fn unlock(self) -> Mutex<T> {
    // Do low-level OS mutex unlock
    self.locked_mutex.sys_mutex.unlock();
    return self.locked_mutex;
  }
}

impl<T> Deref for Locked<T> { ... }
impl<T> DerefMut for Locked<T> { ... }
