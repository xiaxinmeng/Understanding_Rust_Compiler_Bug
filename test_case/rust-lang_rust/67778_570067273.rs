txt
thread 'rustc' panicked at 'no entry found for key', src/libcore/option.rs:1188:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1057
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:195
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:215
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/119307a83e12291a3fc126735d6bd0292c443464/src/liballoc/boxed.rs:1029
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/119307a83e12291a3fc126735d6bd0292c443464/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  14: rust_begin_unwind
             at src/libstd/panicking.rs:376
  15: core::panicking::panic_fmt
             at src/libcore/panicking.rs:84
  16: core::option::expect_failed
             at src/libcore/option.rs:1188
  17: rustc::hir::map::definitions::Definitions::invocation_parent
  18: rustc_resolve::macros::<impl rustc_expand::base::Resolver for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
  19: rustc_expand::expand::MacroExpander::collect_invocations
  20: rustc_expand::expand::MacroExpander::fully_expand_fragment
  21: rustc_expand::expand::MacroExpander::expand_crate
  22: rustc_interface::passes::configure_and_expand_inner::{{closure}}
  23: rustc::util::common::time
  24: rustc_interface::passes::configure_and_expand::{{closure}}
  25: rustc_interface::passes::configure_and_expand
  26: rustc_interface::queries::Queries::expansion
  27: rustc_interface::interface::run_compiler_in_existing_thread_pool
  28: scoped_tls::ScopedKey<T>::set
  29: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
