
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 18446744073709551613', src/tools/clippy/clippy_lints/src/misc_early.rs:511:16
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
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
             at src/libstd/panicking.rs:200
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:214
  10: rustc::util::common::panic_hook
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
  13: rust_begin_unwind
             at src/libstd/panicking.rs:311
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  15: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  16: <clippy_lints::misc_early::MiscEarlyLints as rustc::lint::EarlyLintPass>::check_expr
  17: <rustc::lint::context::EarlyLintPassObjects as rustc::lint::EarlyLintPass>::check_expr
  18: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_expr
  19: syntax::visit::walk_expr
  20: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_expr
  21: syntax::visit::walk_expr
  22: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_expr
  23: syntax::visit::walk_fn
  24: syntax::visit::walk_impl_item
  25: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_impl_item
  26: syntax::visit::walk_item
  27: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_item
  28: syntax::visit::walk_item
  29: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_item
  30: syntax::visit::walk_item
  31: <rustc::lint::context::EarlyContextAndPass<T> as syntax::visit::Visitor>::visit_item
  32: syntax::visit::walk_crate
  33: rustc::lint::context::early_lint_crate
  34: rustc::lint::context::check_ast_crate
  35: rustc::util::common::time
  36: rustc_interface::passes::BoxedResolver::access::{{closure}}
  37: rustc_interface::passes::configure_and_expand::{{closure}}
  38: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
  39: rustc_interface::queries::Query<T>::compute
  40: rustc_interface::queries::Query<T>::compute
  41: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::prepare_outputs
  42: rustc_interface::interface::run_compiler_in_existing_thread_pool
  43: std::thread::local::LocalKey<T>::with
  44: scoped_tls::ScopedKey<T>::set
  45: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack
