
error: internal compiler error: constant in type had an ignored error
 --> src/main.rs:7:5
  |
7 |     stack: MaybeUninit<[u64; N / 8]>
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:356:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/libunwind.rs:97
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:197
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   8: std::panicking::begin_panic
   9: <rustc_errors::Handler as core::ops::drop::Drop>::drop
  10: core::ptr::real_drop_in_place
  11: core::ptr::real_drop_in_place
  12: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  13: core::ptr::real_drop_in_place
  14: rustc_interface::interface::run_compiler_in_existing_thread_pool
  15: std::thread::local::LocalKey<T>::with
  16: scoped_tls::ScopedKey<T>::set
  17: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
