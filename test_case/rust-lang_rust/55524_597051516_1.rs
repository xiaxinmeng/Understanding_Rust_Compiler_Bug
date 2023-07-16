
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
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  12: rust_begin_unwind
             at src/libstd/panicking.rs:380
  13: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:334
  14: rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt>::layout_raw_uncached
  15: rustc::ty::layout::layout_raw
  16: rustc::ty::query::__query_compute::layout_raw
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::layout_raw>::compute
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  20: <rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  21: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  22: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  23: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  24: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  25: core::iter::adapters::process_results
  26: rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt>::layout_raw_uncached
  27: rustc::ty::layout::layout_raw
  28: rustc::ty::query::__query_compute::layout_raw
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::layout_raw>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: <rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  33: rustc_mir::interpret::intrinsics::eval_nullary_intrinsic
  34: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  35: rustc::ty::query::__query_compute::const_eval_validated
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_instance
  40: rustc_mir::interpret::eval_context::InterpCx<M>::const_eval
  41: rustc_mir::interpret::intrinsics::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::emulate_intrinsic
  42: <rustc_mir::const_eval::machine::CompileTimeInterpreter as rustc_mir::interpret::machine::Machine>::call_intrinsic
  43: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_fn_call
  44: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
  45: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
  46: rustc::ty::query::__query_compute::const_eval_raw
  47: rustc::ty::context::tls::with_context::{{closure}}
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc_mir::const_eval::machine::<impl rustc_mir::interpret::eval_context::InterpCx<rustc_mir::const_eval::machine::CompileTimeInterpreter>>::try_eval_const_fn_call
  50: <rustc_mir::const_eval::machine::CompileTimeInterpreter as rustc_mir::interpret::machine::Machine>::find_mir_or_eval_fn
  51: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_fn_call
  52: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
  53: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
  54: rustc::ty::query::__query_compute::const_eval_raw
  55: rustc::ty::context::tls::with_context::{{closure}}
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  57: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  58: rustc::ty::query::__query_compute::const_eval_validated
  59: rustc::ty::context::tls::with_context::{{closure}}
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  61: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  62: rustc::ty::query::__query_compute::const_eval_validated
  63: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  64: rustc::dep_graph::graph::DepGraph::with_task_impl
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  66: rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_poly
  67: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  69: rustc::ty::context::tls::with_context::{{closure}}
  70: rustc_typeck::check::typeck_tables_of
  71: rustc::ty::query::__query_compute::typeck_tables_of
  72: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  73: rustc::dep_graph::graph::DepGraph::with_task_impl
  74: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  75: rustc_typeck::check::check_item_type
  76: rustc::hir::map::Map::visit_item_likes_in_module
  77: rustc_typeck::check::check_mod_item_types
  78: rustc::ty::query::__query_compute::check_mod_item_types
  79: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_item_types>::compute
  80: rustc::dep_graph::graph::DepGraph::with_task_impl
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  83: rustc_typeck::check_crate
  84: rustc_interface::passes::analysis
  85: rustc::ty::query::__query_compute::analysis
  86: rustc::dep_graph::graph::DepGraph::with_task_impl
  87: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  88: rustc::ty::context::tls::enter_global
  89: rustc_interface::interface::run_compiler_in_existing_thread_pool
  90: scoped_tls::ScopedKey<T>::set
  91: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-beta.5 (4e1c5f0e9 2020-02-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z fuel=rustc=0 -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=2 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=y --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [layout_raw] computing layout of `middle::region::FirstStatementIndex`
#1 [layout_raw] computing layout of `middle::region::ScopeData`
#2 [const_eval_validated] const-evaluating + checking `std::intrinsics::size_of`
#3 [const_eval_raw] const-evaluating `std::mem::size_of`
#4 [const_eval_raw] const-evaluating `middle::region::_::{{constant}}#1`
#5 [const_eval_validated] const-evaluating + checking `middle::region::_::{{constant}}#1`
#6 [const_eval_validated] const-evaluating + checking `middle::region::_::{{constant}}#1`
#7 [typeck_tables_of] processing `middle::region::_`
#8 [check_mod_item_types] checking item types in module `middle::region`
#9 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `rustc`.
