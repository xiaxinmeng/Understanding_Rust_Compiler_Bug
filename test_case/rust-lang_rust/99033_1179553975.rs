plain
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
error: failed to run custom build command for `getrandom v0.2.0`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/getrandom-6037786444d07737/build-script-build` (signal: 6, SIGABRT: process abort signal)
  --- stderr
  thread '<unnamed>' panicked at 'attempted to zero-initialize type `libc::unix::linux_like::linux::gnu::b64::x86_64::sigaction`, which is invalid', library/std/src/sys/unix/stack_overflow.rs:108:37
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  fatal runtime error: initialization or cleanup bug
error: failed to run custom build command for `log v0.4.14`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/log-50dcdcb1a7306851/build-script-build` (signal: 6, SIGABRT: process abort signal)
  --- stderr
  thread '<unnamed>' panicked at 'attempted to zero-initialize type `libc::unix::linux_like::linux::gnu::b64::x86_64::sigaction`, which is invalid', library/std/src/sys/unix/stack_overflow.rs:108:37
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  fatal runtime error: initialization or cleanup bug
error: failed to run custom build command for `parking_lot_core v0.8.5`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/parking_lot_core-a5d4c23a761f33d2/build-script-build` (signal: 6, SIGABRT: process abort signal)
  --- stderr
  thread '<unnamed>' panicked at 'attempted to zero-initialize type `libc::unix::linux_like::linux::gnu::b64::x86_64::sigaction`, which is invalid', library/std/src/sys/unix/stack_overflow.rs:108:37
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  fatal runtime error: initialization or cleanup bug
error: failed to run custom build command for `proc-macro-hack v0.5.19`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/proc-macro-hack-325d958db648420b/build-script-build` (signal: 6, SIGABRT: process abort signal)
  --- stderr
  thread '<unnamed>' panicked at 'attempted to zero-initialize type `libc::unix::linux_like::linux::gnu::b64::x86_64::sigaction`, which is invalid', library/std/src/sys/unix/stack_overflow.rs:108:37
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  fatal runtime error: initialization or cleanup bug
