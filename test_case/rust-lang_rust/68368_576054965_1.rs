
error[E0277]: the trait bound `(): Trait<()>` is not satisfied
 --> src/lib.rs:3:1
  |
3 | type Alias<'a, U> = impl Trait<U>;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait<()>` is not implemented for `()`
  |
  = note: the return type of a function must have a statically known size

error: internal compiler error: src/librustc/ty/subst.rs:328: expected type for param #0 in [ReEarlyBound(0, 'a), U]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:882:9
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
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator::check
  21: rustc_hir::intravisit::walk_crate
  22: rustc_typeck::collect::find_opaque_ty_constraints
  23: rustc_typeck::collect::type_of
  24: rustc::ty::query::__query_compute::type_of
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  28: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  29: rustc::hir::map::Map::visit_item_likes_in_module
  30: rustc_typeck::collect::collect_mod_item_types
  31: rustc::ty::query::__query_compute::collect_mod_item_types
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  36: rustc_typeck::check_crate
  37: rustc_interface::passes::analysis
  38: rustc::ty::query::__query_compute::analysis
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: rustc::ty::context::tls::enter_global
  42: rustc_interface::interface::run_compiler_in_existing_thread_pool
  43: scoped_tls::ScopedKey<T>::set
  44: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (1ce08f9d6 2020-01-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] processing `Alias`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors
