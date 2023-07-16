plain
[RUSTC-TIMING] object test:false 6.014
error: unused doc comment
  --> library/std/src/sys/unix/thread_parker.rs:71:5
   |
71 |       /// Use the monotonic clock on other systems.
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
72 |       #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "espidf")))]
73 | /     let (now, dur) = {
74 | |         use super::time::Timespec;
75 | |
76 | |         (Timespec::now(libc::CLOCK_MONOTONIC).t, dur)
   | |______- rustdoc does not generate documentation for statements
   |
   |
   = note: `-D unused-doc-comments` implied by `-D warnings`
   = help: use `//` for a plain comment

error[E0599]: no method named `checked_add_duration` found for struct `timespec` in the current scope
   |
   |
79 |     let timeout = now.checked_add_duration(&dur).map(|t| t.t).unwrap_or(TIMESPEC_MAX);
   |                       ^^^^^^^^^^^^^^^^^^^^ method not found in `timespec`
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 2.656
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:06:37
