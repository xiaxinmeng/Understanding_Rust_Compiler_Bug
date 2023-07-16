rust
   pub fn deterministic_hashing_enabled() -> bool {
       let flags = map::HASHING_FLAGS.load(Ordering::SeqCst);
       (flags & map::DETERMINISTIC_HASHING_ENABLED) != 0
   }
   