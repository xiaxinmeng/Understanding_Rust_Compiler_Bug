
note: tracking was triggered
  --> /home/hyd-dev/Code/rust-lang/rust/library/alloc/src/alloc.rs:86:14
   |
86 |     unsafe { __rust_alloc(layout.size(), layout.align()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rust memory allocated
   |
   = note: inside `std::alloc::alloc` at /home/hyd-dev/Code/rust-lang/rust/library/alloc/src/alloc.rs:86:14
   = note: inside `std::alloc::Global::alloc_impl` at /home/hyd-dev/Code/rust-lang/rust/library/alloc/src/alloc.rs:166:73
   = note: inside `<std::alloc::Global as std::alloc::Allocator>::allocate` at /home/hyd-dev/Code/rust-lang/rust/library/alloc/src/alloc.rs:226:9
   = note: inside `alloc::alloc::exchange_malloc` at /home/hyd-dev/Code/rust-lang/rust/library/alloc/src/alloc.rs:316:11
   = note: inside `std::thread::__OsLocalKeyInner::<std::cell::Cell<usize>>::try_initialize` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/thread/local.rs:703:42
   = note: inside `std::thread::__OsLocalKeyInner::<std::cell::Cell<usize>>::get` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/thread/local.rs:685:22
   = note: inside `std::rt::panic_count::LOCAL_PANIC_COUNT::__getit` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/thread/local.rs:287:26
   = note: inside `std::thread::LocalKey::<std::cell::Cell<usize>>::try_with::<[closure@std::rt::panic_count::increase::{closure#0}], usize>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/thread/local.rs:375:32
   = note: inside `std::thread::LocalKey::<std::cell::Cell<usize>>::with::<[closure@std::rt::panic_count::increase::{closure#0}], usize>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/thread/local.rs:352:9
   = note: inside `std::rt::panic_count::increase` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:263:13
   = note: inside `std::panicking::rust_panic_with_hook` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:588:32
   = note: inside closure at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:542:9
   = note: inside `std::sys_common::backtrace::__rust_end_short_backtrace::<[closure@std::rt::begin_panic<&str>::{closure#0}], !>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/sys_common/backtrace.rs:141:18
   = note: inside `std::rt::begin_panic::<&str>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:541:12
note: inside closure at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panic.rs:28:9
  --> src/main.rs:11:13
   |
11 |     run(&|| panic!());
   |             ^^^^^^^^
note: inside `run` at src/main.rs:7:5
  --> src/main.rs:7:5
   |
7  |     do_panic();
   |     ^^^^^^^^^^
note: inside `main` at src/main.rs:11:5
  --> src/main.rs:11:5
   |
11 |     run(&|| panic!());
   |     ^^^^^^^^^^^^^^^^^
   = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at /home/hyd-dev/Code/rust-lang/rust/library/core/src/ops/function.rs:227:5
   = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/sys_common/backtrace.rs:125:18
   = note: inside closure at /home/hyd-dev/Code/rust-lang/rust/library/std/src/rt.rs:49:18
   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /home/hyd-dev/Code/rust-lang/rust/library/core/src/ops/function.rs:259:13
   = note: inside `std::panicking::r#try::do_call::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:401:40
   = note: inside `std::panicking::r#try::<i32, &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panicking.rs:365:19
   = note: inside `std::panic::catch_unwind::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/panic.rs:433:14
   = note: inside `std::rt::lang_start_internal` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/rt.rs:34:21
   = note: inside `std::rt::lang_start::<()>` at /home/hyd-dev/Code/rust-lang/rust/library/std/src/rt.rs:48:5
   = note: this note originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
