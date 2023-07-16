plain

error[E0432]: unresolved import `crate::sys::unix_sigpipe_attr_specified`
   --> library/std/src/sys/unix/process/process_unix.rs:281:39
    |
281 |         use crate::sys::{self, cvt_r, unix_sigpipe_attr_specified};
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `unix_sigpipe_attr_specified` in `sys`

error: unused imports: `AtomicBool`, `Ordering`
  |
5 | use crate::sync::atomic::{AtomicBool, Ordering};
  |                           ^^^^^^^^^^  ^^^^^^^^
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] std test:false 3.226
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 2 previous errors; 1 warning emitted
