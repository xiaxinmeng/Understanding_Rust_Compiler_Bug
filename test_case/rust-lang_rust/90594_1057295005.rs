
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: <rustc_metadata::creader::CStore as rustc_session::cstore::CrateStore>::def_path_hash_to_def_id
   4: <rustc_middle::ty::context::TyCtxt>::def_path_hash_to_def_id
   5: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
   6: rustc_query_impl::query_callbacks::adt_def::force_from_dep_node
   7: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
   8: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   9: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>
  12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>>
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue
  15: rustc_mir_transform::check_unsafety::unsafety_check_result
  16: <rustc_mir_transform::check_unsafety::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
  17: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>
  18: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::query::UnsafetyCheckResult, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>::{closure#3}>
  19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>>
  20: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
  21: rustc_query_impl::query_callbacks::unsafety_check_result::force_from_dep_node
  22: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
  23: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  24: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  25: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  27: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
  28: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::ty::TyS>
  29: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::ty::TyS>>
  30: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
  31: rustc_typeck::check::check::check_item_type
  32: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck::check::CheckItemTypesVisitor>
  33: rustc_typeck::check::check::check_mod_item_types
  34: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  35: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
  36: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  37: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  38: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#6}::{closure#0}>
  39: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#6}>
  40: rustc_typeck::check_crate
  41: rustc_interface::passes::analysis
  42: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  43: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  44: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  45: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  46: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  47: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  48: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  49: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  50: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65c55bf93 2021-11-23) running on x86_64-apple-darwin

note: compiler flags: -C panic=abort -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [unsafety_check_result] unsafety-checking `results_sink::{closure#0}`
#1 [unsafety_check_result] unsafety-checking `results_sink`
#2 [type_of] computing type of `results_sink::{opaque#0}`
#3 [check_mod_item_types] checking item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack

