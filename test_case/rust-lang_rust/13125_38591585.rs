 rust
struct Mutex<T> {
   lock: RawMutex,
   data: Unsafe<T>
}

impl<T: Send> Share for Mutex<T> {}
