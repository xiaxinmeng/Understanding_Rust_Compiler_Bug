
 Documenting std v0.0.0 (/home/joshua/rustc2/library/std)
error[E0432]: unresolved import `crate::sys::cvt`
 --> library/std/src/sys/unix/ext/net/addr.rs:4:5
  |
4 | use crate::sys::cvt;
  |     ^^^^^^^^^^^^^^^ no `cvt` in `sys`

error[E0432]: unresolved import `libc::c_int`
  --> library/std/src/sys/unix/ext/net/addr.rs:11:13
   |
11 |     pub use libc::c_int;
   |             ^^^^^^^^^^^ no `c_int` in the root

error[E0432]: unresolved import `crate::sys::net::Socket`
  --> library/std/src/sys/unix/ext/net/ancillary.rs:12:5
   |
12 | use crate::sys::net::Socket;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Socket` in `sys::wasm::net`

error[E0432]: unresolved import `libc::c_int`
  --> library/std/src/sys/unix/ext/net/ancillary.rs:18:13
   |
18 |     pub use libc::c_int;
   |             ^^^^^^^^^^^ no `c_int` in the root

error[E0432]: unresolved import `crate::sys::cvt`
  --> library/std/src/sys/unix/ext/net/datagram.rs:26:5
   |
26 | use crate::sys::cvt;
   |     ^^^^^^^^^^^^^^^ no `cvt` in `sys`

error[E0432]: unresolved import `crate::sys::net::Socket`
  --> library/std/src/sys/unix/ext/net/datagram.rs:27:5
   |
27 | use crate::sys::net::Socket;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Socket` in `sys::wasm::net`

error[E0432]: unresolved import `crate::sys::cvt`
 --> library/std/src/sys/unix/ext/net/listener.rs:4:5
  |
4 | use crate::sys::cvt;
  |     ^^^^^^^^^^^^^^^ no `cvt` in `sys`

error[E0432]: unresolved import `crate::sys::net::Socket`
 --> library/std/src/sys/unix/ext/net/listener.rs:5:5
  |
5 | use crate::sys::net::Socket;
  |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Socket` in `sys::wasm::net`

error[E0432]: unresolved import `crate::sys::cvt`
  --> library/std/src/sys/unix/ext/net/stream.rs:28:5
   |
28 | use crate::sys::cvt;
   |     ^^^^^^^^^^^^^^^ no `cvt` in `sys`

error[E0432]: unresolved import `crate::sys::net::Socket`
  --> library/std/src/sys/unix/ext/net/stream.rs:29:5
   |
29 | use crate::sys::net::Socket;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Socket` in `sys::wasm::net`

error[E0412]: cannot find type `c_int` in crate `libc`
  --> library/std/src/sys/unix/ext/net/datagram.rs:51:27
   |
51 | const MSG_NOSIGNAL: libc::c_int = 0x0;
   |                           ^^^^^ not found in `libc`
   |
help: consider importing this type alias
   |
11 | use crate::sys::unix_ext::io::raw::c_int;
   |

error[E0412]: cannot find type `c_int` in crate `libc`
   --> library/std/src/sys/unix/ext/net/datagram.rs:262:22
    |
262 |         flags: libc::c_int,
    |                      ^^^^^ not found in `libc`
    |
help: consider importing this type alias
    |
11  | use crate::sys::unix_ext::io::raw::c_int;
    |

error: Compilation failed, aborting rustdoc

error: aborting due to 13 previous errors
