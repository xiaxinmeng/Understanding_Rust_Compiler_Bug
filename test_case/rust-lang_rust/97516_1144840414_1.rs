rust
let mut atomic = AtomicI32::new(0); // value in new isn't required, since `atomic` was default-init in C++
atomic.store(1,Ordering::Relaxed);
atomic = AtomicI32::new(2); // or `*atomic.get_mut() = 2;`
let val = atomic.fetch_add(1, Ordering::Relaxed); // reads the 1 stored 2 lines above
