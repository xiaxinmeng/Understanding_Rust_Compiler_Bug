rust
let mutex = Mutex::new(());
let mut guard = None;
guard = Some(mutex.lock().unwrap());
drop(guard.take());      // unwrap() removed
mutex.try_lock().unwrap();    // panics "WouldBlock"
