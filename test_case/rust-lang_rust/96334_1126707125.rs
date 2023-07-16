plain
...........................................................iiiiii....................... 1144/1200
........................i...............................
failures:

---- src/os/unix/net/datagram.rs - os::unix::net::datagram::UnixDatagram::set_mark (line 889) stdout ----
error[E0658]: use of unstable library feature 'unix_socket_ancillary_data'
  |
  |
7 |     sock.set_passcred(32 as u32).expect("set_mark function failed");
  |
  = note: see issue #76915 <https://github.com/rust-lang/rust/issues/76915> for more information
  = note: see issue #76915 <https://github.com/rust-lang/rust/issues/76915> for more information
  = help: add `#![feature(unix_socket_ancillary_data)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> src/os/unix/net/datagram.rs:894:23
    |
    |
7   |     sock.set_passcred(32 as u32).expect("set_mark function failed");
    |          ------------ ^^^^^^^^^ expected `bool`, found `u32`
    |          arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/library/std/src/os/unix/net/datagram.rs:870:12
---

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/os/unix/net/stream.rs - os::unix::net::stream::UnixStream::set_mark (line 439) stdout ----
error[E0658]: use of unstable library feature 'unix_socket_ancillary_data'
  |
  |
7 |     sock.set_passcred(32 as u32).expect("set_mark function failed");
  |
  = note: see issue #76915 <https://github.com/rust-lang/rust/issues/76915> for more information
  = note: see issue #76915 <https://github.com/rust-lang/rust/issues/76915> for more information
  = help: add `#![feature(unix_socket_ancillary_data)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> src/os/unix/net/stream.rs:444:23
    |
    |
7   |     sock.set_passcred(32 as u32).expect("set_mark function failed");
    |          ------------ ^^^^^^^^^ expected `bool`, found `u32`
    |          arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/library/std/src/os/unix/net/stream.rs:420:12
