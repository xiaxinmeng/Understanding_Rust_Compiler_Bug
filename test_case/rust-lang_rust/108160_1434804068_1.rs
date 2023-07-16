rs
pub struct MMutex<T>(T);
pub type Mutex<T> = MMutex<T>;
