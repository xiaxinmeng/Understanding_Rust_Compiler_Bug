
error: internal compiler error: broken MIR in DefId(0/0:8 ~ playground[2d7c]::{{impl}}[0]::FUN[0]) (bb0[0]): equate_inputs_and_outputs: `fn() -> Item==fn() -> Item` failed with `NoSolution`
  --> src/lib.rs:12:5
   |
12 |     const FUN: fn() -> Self::Item = || {};
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::begin_panic
   7: <rustc_errors::Handler as core::ops::drop::Drop>::drop
   8: core::ptr::real_drop_in_place
   9: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  10: core::ptr::real_drop_in_place
  11: rustc_interface::interface::run_compiler_in_existing_thread_pool
  12: std::thread::local::LocalKey<T>::with
  13: scoped_tls::ScopedKey<T>::set
  14: syntax::with_globals
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (938d4ffe1 2019-04-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
