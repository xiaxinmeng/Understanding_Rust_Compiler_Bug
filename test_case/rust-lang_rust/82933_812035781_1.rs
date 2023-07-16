rust
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:180:30
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: rustc_middle::hir::map::Map::opt_def_kind
   4: core::ops::function::FnOnce::call_once
   5: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   6: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
   7: rustc_data_structures::stack::ensure_sufficient_stack
   8: rustc_query_system::query::plumbing::force_query_with_job
   9: rustc_query_system::query::plumbing::get_query_impl
  10: rustc_middle::ty::util::<impl rustc_middle::ty::context::TyCtxt>::closure_base_def_id
  11: rustc_typeck::collect::generics_of
  12: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::generics_of>::compute
  13: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  15: rustc_data_structures::stack::ensure_sufficient_stack
  16: rustc_query_system::query::plumbing::force_query_with_job
  17: rustc_query_system::query::plumbing::get_query_impl
  18: rustc_query_system::query::plumbing::ensure_query_impl
  19: rustc_hir::intravisit::walk_expr
  20: rustc_hir::intravisit::walk_expr
  21: rustc_hir::intravisit::walk_expr
  22: rustc_hir::intravisit::walk_stmt
  23: rustc_hir::intravisit::walk_block
  24: rustc_hir::intravisit::walk_expr
  25: rustc_hir::intravisit::walk_stmt
  26: rustc_hir::intravisit::walk_block
  27: rustc_hir::intravisit::walk_body
  28: rustc_hir::intravisit::walk_trait_item
  29: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_trait_item
  30: rustc_middle::hir::map::Map::visit_item_likes_in_module
  31: rustc_typeck::collect::collect_mod_item_types
  32: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_mod_item_types>::compute
  33: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  34: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  35: rustc_data_structures::stack::ensure_sufficient_stack
  36: rustc_query_system::query::plumbing::force_query_with_job
  37: rustc_query_system::query::plumbing::get_query_impl
  38: rustc_query_system::query::plumbing::ensure_query_impl
  39: rustc_session::session::Session::track_errors
  40: rustc_typeck::check_crate
  41: rustc_interface::passes::analysis
  42: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  43: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  45: rustc_data_structures::stack::ensure_sufficient_stack
  46: rustc_query_system::query::plumbing::force_query_with_job
  47: rustc_query_system::query::plumbing::get_query_impl
  48: rustc_interface::passes::QueryContext::enter
  49: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  50: rustc_span::with_source_map
  51: rustc_interface::interface::create_compiler_and_run
  52: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 (2fd73fabe 2021-03-23) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [opt_def_kind] looking up definition kind of `s4module::kotd::KotdDB::count_wins::{closure#0}`
#1 [generics_of] computing generics of `s4module::kotd::KotdDB::count_wins::{closure#0}::{closure#0}`
#2 [collect_mod_item_types] collecting item types in module `s4module::kotd`
#3 [analysis] running analysis passes on this crate
end of query stack
