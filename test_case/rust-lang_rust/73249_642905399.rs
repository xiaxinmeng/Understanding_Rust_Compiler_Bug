
error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:37: could not fully normalize `<A as sealed::SimdArray>::Tuple`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
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
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::bug_fmt
  20: rustc_middle::ty::context::GlobalCtxt::enter_local
  21: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
  22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::normalize_generic_arg_after_erasing_regions>::compute
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  27: rustc_middle::ty::AdtDef::transparent_newtype_field
  28: rustc_lint::types::ImproperCTypesVisitor::check_type_for_ffi
  29: rustc_lint::types::ImproperCTypesVisitor::check_type_for_ffi_and_report_errors
  30: <rustc_lint::types::ImproperCTypes as rustc_lint::passes::LateLintPass>::check_foreign_item
  31: rustc_hir::intravisit::walk_item
  32: rustc_hir::intravisit::Visitor::visit_nested_item
  33: rustc_hir::intravisit::walk_mod
  34: rustc_lint::late::late_lint_mod
  35: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::lint_mod>::compute
  36: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  37: rustc_data_structures::stack::ensure_sufficient_stack
  38: rustc_query_system::query::plumbing::get_query_impl
  39: rustc_query_system::query::plumbing::ensure_query_impl
  40: rustc_session::utils::<impl rustc_session::session::Session>::time
  41: std::panicking::try
  42: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  43: rustc_session::utils::<impl rustc_session::session::Session>::time
  44: rustc_interface::passes::analysis
  45: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  46: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  47: rustc_query_system::query::plumbing::get_query_impl
  48: rustc_middle::ty::context::tls::enter_global
  49: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  50: rustc_span::with_source_map
  51: rustc_interface::interface::run_compiler_in_existing_thread_pool
  52: scoped_tls::ScopedKey<T>::set
