
$ cargo clippy
    Checking zoa_test v0.1.0 (/Users/mahrens/ws/ports/zfsonlinux/cmd/zfs_object_agent/client)
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
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
  12: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>, core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_mir_transform::const_prop::ConstProp as rustc_middle::mir::MirPass>::run_pass
  15: rustc_mir_transform::run_passes
  16: rustc_mir_transform::optimized_mir
  17: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>
  18: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::Body, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>::{closure#3}>
  19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  21: <clippy_lints::redundant_clone::RedundantClone as rustc_lint::passes::LateLintPass>::check_fn
  22: <rustc_lint::late::LateLintPassObjects as rustc_lint::passes::LateLintPass>::check_fn
  23: <rustc_lint::late::LateContextAndPass<rustc_lint::late::LateLintPassObjects> as rustc_hir::intravisit::Visitor>::visit_fn
  24: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::late::LateLintPassObjects>>
  25: <rustc_lint::late::LateContextAndPass<rustc_lint::late::LateLintPassObjects> as rustc_hir::intravisit::Visitor>::visit_nested_item
  26: rustc_lint::late::late_lint_pass_crate::<rustc_lint::late::LateLintPassObjects>
  27: rustc_lint::late::late_lint_crate::<rustc_lint::BuiltinCombinedLateLintPass>
  28: rustc_data_structures::sync::join::<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#0}::{closure#3}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#0}::{closure#3}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  29: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  30: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
  31: rustc_interface::passes::analysis
  32: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  33: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  35: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  36: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  37: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  38: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  39: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.58 (db9d1b20 2022-01-20)

query stack during panic:
#0 [layout_of] computing layout of `impl core::future::future::Future<Output = [async output]>`
#1 [optimized_mir] optimizing MIR for `main`
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `zoa_test`
