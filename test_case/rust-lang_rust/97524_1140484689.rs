plain
   Compiling addr2line v0.16.0
error[E0308]: mismatched types
    --> library/std/src/thread/mod.rs:1248:23
     |
1248 |         Arc::into_raw(self.inner) as *const ()
     |                       ^^^^^^^^^^ expected struct `Arc`, found struct `Pin`
     = note: expected struct `Arc<_>`
     = note: expected struct `Arc<_>`
                found struct `Pin<Arc<thread::Inner>>`
error[E0308]: mismatched types
    --> library/std/src/thread/mod.rs:1270:34
     |
     |
1270 |         unsafe { Thread { inner: Arc::from_raw(ptr as *const Inner) } }
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Pin`, found struct `Arc`
     |
     = note: expected struct `Pin<Arc<_>>`
                found struct `Arc<_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:19
