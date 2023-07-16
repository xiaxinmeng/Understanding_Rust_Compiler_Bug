
   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'byte index 30 is not a char boundary; it is inside 'ö' (bytes 29..31) of `struct X {}

const Y: X = X("ö");`', src/libcore/str/mod.rs:2034:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
   9: rust_begin_unwind
             at src/libstd/panicking.rs:311
  10: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  11: core::str::slice_error_fail
             at src/libcore/str/mod.rs:0
  12: core::str::traits::<impl core::slice::SliceIndex<str> for core::ops::range::RangeFrom<usize>>::index::{{closure}}
  13: syntax::source_map::SourceMap::span_to_snippet
  14: rustc_resolve::diagnostics::<impl rustc_resolve::Resolver>::smart_resolve_context_dependent_help::{{closure}}
  15: rustc_resolve::diagnostics::<impl rustc_resolve::Resolver>::smart_resolve_report_errors
  16: rustc_resolve::Resolver::smart_resolve_path_fragment::{{closure}}
  17: rustc_resolve::Resolver::smart_resolve_path_fragment
  18: rustc_resolve::Resolver::smart_resolve_path
  19: rustc_resolve::Resolver::resolve_expr
  20: rustc_resolve::Resolver::resolve_expr
  21: rustc_resolve::Resolver::resolve_item
  22: rustc_resolve::Resolver::resolve_crate
  23: rustc::util::common::time
  24: rustc_interface::passes::configure_and_expand_inner
  25: rustc_interface::passes::configure_and_expand::{{closure}}
  26: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  27: rustc_interface::passes::configure_and_expand
  28: rustc_interface::queries::Query<T>::compute
  29: rustc_interface::queries::Query<T>::compute
  30: rustc_interface::queries::Query<T>::compute
  31: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::prepare_outputs
  32: rustc_interface::interface::run_compiler_in_existing_thread_pool
  33: std::thread::local::LocalKey<T>::with
  34: scoped_tls::ScopedKey<T>::set
  35: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0 (eae3437df 2019-08-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
