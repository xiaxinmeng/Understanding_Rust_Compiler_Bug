
thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:367:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:217
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:530
  12: std::panicking::begin_panic
  13: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
  14: core::ptr::drop_in_place
  15: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  16: core::ptr::drop_in_place
  17: rustc_span::with_source_map
  18: rustc_interface::interface::create_compiler_and_run
  19: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0 (04488afe3 2020-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
