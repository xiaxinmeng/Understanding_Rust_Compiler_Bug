
  Compiling playground v0.0.1 (/playground)
error[E0658]: existential types are unstable
 --> src/lib.rs:8:5
  |
8 |     existential type Item: Fn() -> Self::Item;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/34511
  = help: add #![feature(existential_type)] to the crate attributes to enable

thread 'rustc' panicked at 'instantiated twice: DefId(0/0:7 ~ playground[2d7c]::{{impl}}[0]::Item[0])/OpaqueTypeDecl { substs: [], concrete_ty: _, has_required_region_bounds: false, origin: ExistentialType }', src/librustc_typeck/check/mod.rs:2354:13
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:381
   7: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:336
   8: rustc_typeck::check::FnCtxt::instantiate_opaque_types_from_value
   9: rustc_typeck::check::check_fn
  10: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  11: rustc_typeck::check::FnCtxt::check_expr_kind
  12: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  13: rustc_typeck::check::FnCtxt::check_block_with_expected
  14: rustc_typeck::check::FnCtxt::check_expr_kind
  15: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  16: rustc_typeck::check::FnCtxt::check_return_expr
  17: rustc_typeck::check::check_fn
  18: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  19: rustc_typeck::check::FnCtxt::check_expr_kind
  20: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  21: rustc::ty::context::GlobalCtxt::enter_local
  22: rustc_typeck::check::typeck_tables_of
  23: rustc::ty::query::__query_compute::typeck_tables_of
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc_typeck::collect::find_existential_constraints::ConstraintLocator::check
  28: rustc::hir::intravisit::Visitor::visit_nested_impl_item
  29: rustc::hir::intravisit::walk_item
  30: rustc_typeck::collect::find_existential_constraints
  31: rustc_typeck::collect::checked_type_of
  32: rustc_typeck::collect::type_of
  33: rustc::ty::query::__query_compute::type_of
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_impl_item
  38: rustc::hir::map::Map::visit_item_likes_in_module
  39: rustc_typeck::collect::collect_mod_item_types
  40: rustc::ty::query::__query_compute::collect_mod_item_types
  41: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  42: rustc::dep_graph::graph::DepGraph::with_task_impl
  43: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  44: rustc_typeck::check_crate::{{closure}}::{{closure}}
  45: rustc::util::common::time
  46: rustc_typeck::check_crate
  47: rustc_interface::passes::analysis
  48: rustc::ty::query::__query_compute::analysis
  49: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  50: rustc::dep_graph::graph::DepGraph::with_task_impl
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  52: rustc::ty::context::tls::enter_global
  53: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  54: rustc_interface::passes::create_global_ctxt::{{closure}}
  55: rustc_interface::interface::run_compiler_in_existing_thread_pool
  56: std::thread::local::LocalKey<T>::with
  57: scoped_tls::ScopedKey<T>::set
  58: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `<() as Bug>::FUN`
#1 [type_of] processing `<() as Bug>::Item`
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (938d4ffe1 2019-04-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
