
error: unknown start of token: \u{a7}
 --> src/main.rs:2:4
  |
2 | M (§& u8)}
  |    ^

thread 'rustc' panicked at 'byte index 4 is not a char boundary; it is inside '§' (bytes 3..5) of `M (§& u8)}`', src/libcore/str/mod.rs:2154:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
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
             at src/libstd/panicking.rs:474
  12: rust_begin_unwind
             at src/libstd/panicking.rs:378
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::str::slice_error_fail
             at src/libcore/str/mod.rs:0
  15: core::str::traits::<impl core::slice::SliceIndex<str> for core::ops::range::RangeTo<usize>>::index::{{closure}}
  16: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  17: <core::iter::adapters::Cloned<I> as core::iter::traits::iterator::Iterator>::try_fold
  18: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  19: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  20: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
  21: rustc_errors::HandlerInner::emit_diagnostic
  22: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  23: rustc_resolve::lifetimes::LifetimeContext::resolve_elided_lifetimes
  24: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_lifetime
  25: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_ty
  26: rustc_hir::intravisit::walk_enum_def
  27: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_item
  28: rustc_resolve::lifetimes::resolve_lifetimes
  29: rustc::ty::query::__query_compute::resolve_lifetimes
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: core::ops::function::FnOnce::call_once
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc::ty::context::TyCtxt::object_lifetime_defaults
  36: rustc_typeck::collect::generics_of
  37: rustc::ty::query::__query_compute::generics_of
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::generics_of>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  42: rustc::hir::map::Map::visit_item_likes_in_module
  43: rustc_typeck::collect::collect_mod_item_types
  44: rustc::ty::query::__query_compute::collect_mod_item_types
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  49: rustc_session::session::Session::track_errors
  50: rustc_typeck::check_crate
  51: rustc_interface::passes::analysis
  52: rustc::ty::query::__query_compute::analysis
  53: rustc::dep_graph::graph::DepGraph::with_task_impl
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  55: rustc::ty::context::tls::enter_global
  56: rustc_interface::interface::run_compiler_in_existing_thread_pool
  57: scoped_tls::ScopedKey<T>::set
  58: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (a1912f2e8 2020-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolve_lifetimes] resolving lifetimes
#1 [object_lifetime_defaults_map] looking up lifetime defaults for a region
#2 [generics_of] processing `F`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
error: could not compile `bisectit`.
