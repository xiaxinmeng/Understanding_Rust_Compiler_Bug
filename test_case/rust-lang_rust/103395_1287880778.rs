
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: rustc_hir_analysis::collect::lifetimes::object_lifetime_default
   4: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>::{closure#0}, rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>
   5: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>
   6: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>>
   7: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::object_lifetime_default, rustc_query_impl::plumbing::QueryCtxt>
   8: rustc_query_impl::plumbing::force_from_dep_node::<rustc_query_impl::queries::object_lifetime_default>
   9: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  12: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  13: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  15: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  16: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
  17: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::generics::GenericPredicates>
  18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::GenericPredicates>>
  19: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::predicates_of, rustc_query_impl::plumbing::QueryCtxt>
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::predicates_of
  21: <rustc_privacy::ReachEverythingInTheInterfaceVisitor>::predicates
  22: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
  23: rustc_hir::intravisit::walk_mod::<rustc_privacy::EmbargoVisitor>
  24: rustc_privacy::privacy_access_levels
  25: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), &rustc_middle::middle::privacy::AccessLevels>::{closure#0}, &rustc_middle::middle::privacy::AccessLevels>
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), &rustc_middle::middle::privacy::AccessLevels>
  27: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), &rustc_middle::middle::privacy::AccessLevels>>
  28: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::privacy_access_levels, rustc_query_impl::plumbing::QueryCtxt>
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::privacy_access_levels
  30: rustc_passes::stability::check_unused_or_stable_features
  31: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  32: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#0}::{closure#2}>>
  33: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}>
  34: rustc_interface::passes::analysis
  35: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>  36: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  39: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  40: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  41: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  42: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  43: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C incremental=[REDACTED] -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden
   Compiling unic-char-range v0.9.0

query stack during panic:
#0 [object_lifetime_default] looking up lifetime defaults for generic parameter `LabelText`
#1 [predicates_of] computing predicates of `LabelText`
#2 [privacy_access_levels] checking privacy access levels
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `rustc_graphviz`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:20
