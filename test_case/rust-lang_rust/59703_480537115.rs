
$ RUST_BACKTRACE=1 cargo doc
 Documenting nom v4.2.2 (file:///app/nom)
thread '<unnamed>' panicked at 'failed to generate documentation: Error { file: "/app/nom/target/doc/aliases.js", error: Os { code: 28, kind: Other, message: "No space left on device" } }', libcore/result.rs:945:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:511
   5: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:426
   6: rust_begin_unwind
             at libstd/panicking.rs:337
   7: core::panicking::panic_fmt
             at libcore/panicking.rs:92
   8: core::result::unwrap_failed
             at /checkout/src/libcore/macros.rs:26
   9: syntax::with_globals
             at /checkout/src/libcore/result.rs:809
             at librustdoc/lib.rs:524
             at librustdoc/lib.rs:741
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
             at /checkout/src/libsyntax/lib.rs:97
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
             at /checkout/src/libsyntax/lib.rs:96
  10: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at librustdoc/lib.rs:640
             at /checkout/src/librustc_driver/lib.rs:1625
             at /checkout/src/libstd/panic.rs:308
  11: std::panicking::try::do_call
             at /checkout/src/libstd/panicking.rs:310
  12: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  13: rustc_driver::monitor
             at /checkout/src/libstd/panicking.rs:289
             at /checkout/src/libstd/panic.rs:392
             at /checkout/src/librustc_driver/lib.rs:1547
             at /checkout/src/librustc_driver/lib.rs:1558
             at /checkout/src/librustc_driver/lib.rs:1624
  14: rustdoc::rust_input
             at librustdoc/lib.rs:640
  15: rustdoc::main_args
             at librustdoc/lib.rs:562
             at librustdoc/lib.rs:517
  16: syntax::with_globals
             at librustdoc/lib.rs:110
             at /checkout/src/libcore/option.rs:414
             at librustdoc/lib.rs:110
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
             at /checkout/src/libsyntax/lib.rs:97
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
             at /checkout/src/libsyntax/lib.rs:96
  17: std::panicking::try::do_call
             at /checkout/src/libstd/thread/mod.rs:409
             at /checkout/src/libstd/panic.rs:308
             at /checkout/src/libstd/panicking.rs:310
  18: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  19: <F as alloc::boxed::FnBox<A>>::call_box
             at /checkout/src/libstd/panicking.rs:289
             at /checkout/src/libstd/panic.rs:392
             at /checkout/src/libstd/thread/mod.rs:408
             at /checkout/src/liballoc/boxed.rs:640
  20: std::sys_common::thread::start_thread
             at /checkout/src/liballoc/boxed.rs:650
             at libstd/sys_common/thread.rs:24
  21: std::sys::unix::thread::Thread::new::thread_start
             at libstd/sys/unix/thread.rs:90
  22: start_thread
  23: clone

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0 (9634041f0 2018-07-30) running on x86_64-unknown-linux-gnu

error: Could not document `nom`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name nom src/lib.rs -o /app/nom/target/doc --cfg feature="alloc" --cfg feature="default" --cfg feature="memchr" --cfg feature="std" -L dependency=/app/nom/target/debug/deps --extern memchr=/app/nom/target/debug/deps/libmemchr-b2cd595b80edfc3d.rmeta --cfg stable_i128` (exit code: 101)
