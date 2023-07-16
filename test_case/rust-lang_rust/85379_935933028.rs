plain
[RUSTC-TIMING] gimli test:false 4.046
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/datagram.rs:142:17
    |
142 |                 *socket.0.as_inner(),
    |                 ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/datagram.rs:257:17
    |
    |
257 |                 *self.0.as_inner(),
    |                 ^^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/datagram.rs:571:17
    |
    |
571 |                 *self.0.as_inner(),
    |                 ^^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/listener.rs:113:17
    |
    |
113 |                 *inner.as_inner(),
    |                 ^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/listener.rs:117:30
    |
    |
117 |             cvt(libc::listen(*inner.as_inner(), 128))?;
    |                              ^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
error[E0308]: mismatched types
   --> library/std/src/os/unix/net/stream.rs:138:17
    |
    |
138 |                 *inner.as_inner(),
    |                 ^^^^^^^^^^^^^^^^^ expected `i32`, found struct `FileDesc`
For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] std test:false 1.904
error: could not compile `std` due to 6 previous errors
Build completed unsuccessfully in 0:00:21
