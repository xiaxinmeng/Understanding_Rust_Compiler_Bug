diff
- use parking_lot::Mutex;
+ type Mutex<T> = lock_api::Mutex<parking_lot::RawMutex, T>;
