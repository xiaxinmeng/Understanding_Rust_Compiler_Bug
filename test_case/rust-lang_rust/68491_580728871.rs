
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> src/libstd/sys/sgx/rwlock.rs:16:5
   |
16 |     mem::transmute::<RWLock, [u8; 128]>(r);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: source type: `sys::sgx::rwlock::RWLock` (1152 bits)
   = note: target type: `[u8; 128]` (1024 bits)
