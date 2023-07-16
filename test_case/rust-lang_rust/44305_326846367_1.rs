
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:380
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic_new
             at /checkout/src/libstd/panicking.rs:553
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:521
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:497
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:92
   9: core::panicking::panic
             at /checkout/src/libcore/panicking.rs:51
  10: <core::option::Option<T>>::unwrap
  11: bootstrap::compile::run_cargo
  12: <bootstrap::compile::Std as bootstrap::builder::Step>::run
  13: bootstrap::builder::Builder::ensure
  14: <bootstrap::compile::Test as bootstrap::builder::Step>::run
  15: bootstrap::builder::Builder::ensure
  16: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
  17: bootstrap::builder::Builder::ensure
  18: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  19: bootstrap::builder::StepDescription::maybe_run
  20: bootstrap::builder::StepDescription::run
  21: bootstrap::builder::Builder::run
  22: bootstrap::Build::build
  23: bootstrap::main
  24: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  25: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:458
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:59
  26: main
  27: __libc_start_main
  28: _start
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/home/jessicah/rust/src/bootstrap/bootstrap.py", line 761, in main
    bootstrap()
  File "/home/jessicah/rust/src/bootstrap/bootstrap.py", line 752, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/jessicah/rust/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
