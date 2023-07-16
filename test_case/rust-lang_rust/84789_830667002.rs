
error: internal compiler error: compiler/rustc_privacy/src/lib.rs:272:17: unexpected type: FreshTy(0)

thread 'rustc' panicked at 'Box<Any>', /rustc/42816d61ead7e46d462df997958ccfd514f8c21c/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
   8: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
   9: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
  10: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  11: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::sty::Binder<T>>::super_visit_with
  12: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
  13: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  14: rustc_privacy::ReachEverythingInTheInterfaceVisitor::ty
  15: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
  16: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_mod
  17: rustc_hir::intravisit::walk_crate
  18: rustc_privacy::privacy_access_levels
  19: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::force_query_with_job
  23: rustc_query_system::query::plumbing::get_query_impl
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::privacy_access_levels
  25: std::panic::catch_unwind
  26: rustc_session::utils::<impl rustc_session::session::Session>::time
  27: rustc_interface::passes::analysis
  28: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  30: rustc_data_structures::stack::ensure_sufficient_stack
  31: rustc_query_system::query::plumbing::force_query_with_job
  32: rustc_query_system::query::plumbing::get_query_impl
  33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  34: rustc_interface::passes::QueryContext::enter
  35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  36: rustc_span::with_source_map
  37: rustc_interface::interface::create_compiler_and_run
  38: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (42816d61e 2021-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [privacy_access_levels] privacy access levels
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Box<Any>', /rustc/42816d61ead7e46d462df997958ccfd514f8c21c/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
   8: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
   9: <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold
  10: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
  11: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  12: <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold
  13: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::sty::Binder<T>>::super_visit_with
  14: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
  15: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  16: <rustc_privacy::TypePrivacyVisitor as rustc_hir::intravisit::Visitor>::visit_ty
  17: rustc_hir::intravisit::walk_item
  18: rustc_privacy::check_mod_privacy
  19: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_query_system::query::plumbing::force_query_with_job
  22: rustc_query_system::query::plumbing::get_query_impl
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_privacy
  24: rustc_session::utils::<impl rustc_session::session::Session>::time
  25: rustc_session::utils::<impl rustc_session::session::Session>::time
  26: rustc_interface::passes::analysis
  27: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::force_query_with_job
  31: rustc_query_system::query::plumbing::get_query_impl
  32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  33: rustc_interface::passes::QueryContext::enter
  34: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  35: rustc_span::with_source_map
  36: rustc_interface::interface::create_compiler_and_run
  37: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (42816d61e 2021-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_mod_privacy] checking privacy in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
