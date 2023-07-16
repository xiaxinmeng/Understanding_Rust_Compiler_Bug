bash
$ RUST_BACKTRACE=1 rustc test.rs
thread 'rustc' panicked at 'no entry found for key', src/libcore/option.rs:1187:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:373
  13: rust_begin_unwind
             at src/libstd/panicking.rs:302
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:82
  15: core::option::expect_failed
             at src/libcore/option.rs:1187
  16: rustc::hir::map::def_collector::DefCollector::collect_field
  17: syntax_expand::expand::AstFragment::visit_with
  18: rustc_resolve::macros::<impl syntax_expand::base::Resolver for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
  19: syntax_expand::expand::MacroExpander::collect_invocations
  20: syntax_expand::expand::MacroExpander::fully_expand_fragment
  21: syntax_expand::expand::MacroExpander::expand_crate
  22: rustc_interface::passes::configure_and_expand_inner::{{closure}}
  23: rustc_interface::passes::configure_and_expand_inner
  24: rustc_interface::passes::configure_and_expand::{{closure}}
  25: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  26: rustc_interface::passes::configure_and_expand
  27: rustc_interface::queries::Query<T>::compute
  28: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::expansion
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: std::thread::local::LocalKey<T>::with
  31: scoped_tls::ScopedKey<T>::set
  32: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (3e525e3f6 2019-11-18) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
