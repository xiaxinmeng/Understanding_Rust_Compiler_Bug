
error: Miri evaluation error: entering unreachable code
  --> src/main.rs:10:13
   |
10 |     let _ = PrintName::VOID;
   |             ^^^^^^^^^^^^^^^ Miri evaluation error: entering unreachable code
   |
   = note: inside call to `main` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:67:34
   = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:73
   = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/sys_common/backtrace.rs:136:5
   = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6014 ~ std[74ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:52:13
   = note: inside call to closure at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:292:40
   = note: inside call to `std::panicking::r#try::do_call::<[closure@DefId(1:6013 ~ std[74ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:270:13
   = note: inside call to `std::panicking::r#try::<i32, [closure@DefId(1:6013 ~ std[74ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panic.rs:394:14
   = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6013 ~ std[74ff]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:51:25
   = note: inside call to `std::rt::lang_start_internal` at /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:67:5
   = note: inside call to `std::rt::lang_start::<()>`

error: aborting due to previous error

error: could not compile `playground`.
