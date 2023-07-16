rust
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::begin_panic
   7: <rustc_errors::Handler as core::ops::drop::Drop>::drop
   8: core::ptr::drop_in_place
   9: core::ptr::drop_in_place
  10: core::ptr::drop_in_place
  11: syntax::with_globals
  12: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  13: rustc_driver::run
  14: rustc_driver::main
  15: std::rt::lang_start::{{closure}}
  16: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  17: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  18: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:398
             at libstd/rt.rs:58
  19: main
  20: __libc_start_main
  21: <unknown>
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (0b9f19dff 2018-11-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
