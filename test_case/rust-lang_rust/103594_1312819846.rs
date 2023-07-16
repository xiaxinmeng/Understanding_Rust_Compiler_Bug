plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e4d6307c633c954971f3ca7876d4f29f3fe83614 and 3c37b7ef61075b734bb4219152c8ddc5900344df
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: clock_gettime
+  --> RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   |
+LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
+   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::time::inner::<impl std::sys::PLATFORM::time::Timespec>::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::time::inner::Instant::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::time::Instant::now` at RUSTLIB/std/src/time.rs:LL:CC
+  --> $DIR/sync.rs:LL:CC
+   |
+LL |     let start = Instant::now();
+   |                 ^^^^^^^^^^^^^^
---
full stderr:
error: unsupported operation: can't call foreign function: clock_gettime
  --> /checkout/library/std/src/sys/unix/time.rs:357:26
   |
LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::sys::unix::time::inner::<impl std::sys::unix::time::Timespec>::now` at /checkout/library/std/src/sys/unix/time.rs:357:26
   = note: inside `std::sys::unix::time::inner::Instant::now` at /checkout/library/std/src/sys/unix/time.rs:296:26
   = note: inside `std::time::Instant::now` at /checkout/library/std/src/time.rs:276:17
  --> tests/pass/concurrency/sync.rs:197:17
   |
LL |     let start = Instant::now();
   |                 ^^^^^^^^^^^^^^
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: clock_gettime
+  --> RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   |
+LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
+   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::time::inner::<impl std::sys::PLATFORM::time::Timespec>::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::time::inner::Instant::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::time::Instant::now` at RUSTLIB/std/src/time.rs:LL:CC
+  --> $DIR/time-with-isolation.rs:LL:CC
+   |
+   |
+LL |     let now1 = Instant::now();
+note: inside `main` at $DIR/time-with-isolation.rs:LL:CC
+  --> $DIR/time-with-isolation.rs:LL:CC
+   |
+LL |     test_time_passes();
---
full stderr:
error: unsupported operation: can't call foreign function: clock_gettime
  --> /checkout/library/std/src/sys/unix/time.rs:357:26
   |
LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::sys::unix::time::inner::<impl std::sys::unix::time::Timespec>::now` at /checkout/library/std/src/sys/unix/time.rs:357:26
   = note: inside `std::sys::unix::time::inner::Instant::now` at /checkout/library/std/src/sys/unix/time.rs:296:26
   = note: inside `std::time::Instant::now` at /checkout/library/std/src/time.rs:276:17
  --> tests/pass/shims/time-with-isolation.rs:14:16
   |
   |
LL |     let now1 = Instant::now();
note: inside `main` at tests/pass/shims/time-with-isolation.rs:33:5
  --> tests/pass/shims/time-with-isolation.rs:33:5
   |
LL |     test_time_passes();
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: clock_gettime
+  --> RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   |
+LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
+   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::time::inner::<impl std::sys::PLATFORM::time::Timespec>::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::time::inner::<impl std::sys::PLATFORM::time::SystemTime>::now` at RUSTLIB/std/src/sys/PLATFORM/time.rs:LL:CC
+   = note: inside `std::time::SystemTime::now` at RUSTLIB/std/src/time.rs:LL:CC
+  --> $DIR/time.rs:LL:CC
+   |
+   |
+LL |     let now1 = SystemTime::now();
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
+error: aborting due to previous error
---
full stderr:
error: unsupported operation: can't call foreign function: clock_gettime
  --> /checkout/library/std/src/sys/unix/time.rs:357:26
   |
LL |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: clock_gettime
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::sys::unix::time::inner::<impl std::sys::unix::time::Timespec>::now` at /checkout/library/std/src/sys/unix/time.rs:357:26
   = note: inside `std::sys::unix::time::inner::<impl std::sys::unix::time::SystemTime>::now` at /checkout/library/std/src/sys/unix/time.rs:323:29
   = note: inside `std::time::SystemTime::now` at /checkout/library/std/src/time.rs:496:20
  --> tests/pass/shims/time.rs:20:16
   |
   |
LL |     let now1 = SystemTime::now();

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
