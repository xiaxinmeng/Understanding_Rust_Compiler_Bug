
thread 'toby_snd_34eb687f-749c-4412-99df-e92080069107' panicked at 'assertion failed: `(left == right)`
  left: `4399890432`,
 right: `0`', libstd/sync/mpsc/shared.rs:253:13


stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:205
   3: std::panicking::default_hook
             at libstd/panicking.rs:221
   4: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at libstd/panicking.rs:457
   5: std::panicking::try::do_call
             at libstd/panicking.rs:344
   6: <core::fmt::Arguments<'a> as core::clone::Clone>::clone
             at /Users/travis/build/rust-lang/rust/src/libstd/macros.rs:75
   7: <core::fmt::Arguments<'a> as core::clone::Clone>::clone
             at /Users/travis/build/rust-lang/rust/src/libstd/sync/mpsc/shared.rs:232
   8: <std::sys::unix::time::inner::Instant as core::cmp::PartialOrd>::ge
             at /Users/travis/build/rust-lang/rust/src/libstd/sync/mpsc/mod.rs:1387
   9: <std::sys::unix::time::inner::Instant as core::cmp::PartialOrd>::ge
             at /Users/travis/build/rust-lang/rust/src/libstd/sync/mpsc/mod.rs:1300
  10: tobytcp::TobySender::send_data
             at /Users/gorup/.cargo/registry/src/github.com-1ecc6299db9ec823/tobytcp-0.9.3/src/lib.rs:45
  11: tobytcp::TobyMessenger::start::{{closure}}
             at /Users/gorup/.cargo/registry/src/github.com-1ecc6299db9ec823/tobytcp-0.9.3/src/lib.rs:257
  12: std::sys_common::mutex::Mutex::destroy
             at /Users/travis/build/rust-lang/rust/src/libstd/sys_common/backtrace.rs:136
  13: core::sync::atomic::fence
             at /Users/travis/build/rust-lang/rust/src/libstd/thread/mod.rs:409
  14: core::ptr::swap_nonoverlapping_bytes
             at /Users/travis/build/rust-lang/rust/src/libstd/panic.rs:296
  15: std::sync::mpsc::blocking::SignalToken::cast_from_usize
             at /Users/travis/build/rust-lang/rust/src/libstd/panicking.rs:304
  16: panic_unwind::dwarf::eh::read_encoded_pointer
             at libpanic_unwind/lib.rs:105
  17: std::sync::mpsc::blocking::SignalToken::cast_from_usize
             at /Users/travis/build/rust-lang/rust/src/libstd/panicking.rs:283
  18: std::sync::mpsc::blocking::SignalToken::cast_from_usize
             at /Users/travis/build/rust-lang/rust/src/libstd/panic.rs:361
  19: core::sync::atomic::fence
             at /Users/travis/build/rust-lang/rust/src/libstd/thread/mod.rs:408
  20: core::str::from_utf8_unchecked_mut
             at /Users/travis/build/rust-lang/rust/src/liballoc/boxed.rs:637
  21: std::sys_common::thread::start_thread
             at /Users/travis/build/rust-lang/rust/src/liballoc/boxed.rs:647
             at libstd/sys_common/thread.rs:24
  22: std::sys::unix::thread::Thread::new::thread_start
             at libstd/sys/unix/thread.rs:90
  23: _pthread_body
  24: _pthread_start
