
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_trait_selection::traits::util::get_vtable_index_of_object_method::<()>
   8: rustc_ty_utils::instance::inner_resolve_instance
   9: rustc_ty_utils::instance::resolve_instance
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>
  11: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::resolve_instance, rustc_query_impl::plumbing::QueryCtxt>
  12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  13: <rustc_middle::ty::instance::Instance>::resolve
  14: <rustc_mir_build::lints::Search>::is_recursive_call
  15: <rustc_data_structures::graph::iterate::TriColorDepthFirstSearch<&rustc_middle::mir::Body>>::run_from_start::<rustc_mir_build::lints::Search>
  16: rustc_mir_build::lints::check
  17: <rustc_infer::infer::InferCtxtBuilder>::enter::<rustc_middle::mir::Body, rustc_mir_build::build::mir_build::{closure#1}>
  18: rustc_mir_build::build::mir_built
  19: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>
  20: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>::{closure#3}>
  21: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_built
  23: rustc_mir_transform::check_unsafety::unsafety_check_result
  24: <rustc_mir_transform::check_unsafety::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
  25: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>
  26: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::query::UnsafetyCheckResult, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>::{closure#3}>
  27: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>>
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
  29: rustc_mir_transform::mir_const
  30: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>
  31: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>::{closure#3}>
  32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
  33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_const
  34: rustc_mir_transform::mir_promoted
  35: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, (&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, &rustc_data_structures::steal::Steal<rustc_index::vec::IndexVec<rustc_middle::mir::Promoted, rustc_middle::mir::Body>>)>
  36: rustc_data_structures::stack::ensure_sufficient_stack::<((&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, &rustc_data_structures::steal::Steal<rustc_index::vec::IndexVec<rustc_middle::mir::Promoted, rustc_middle::mir::Body>>), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, (&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, &rustc_data_structures::steal::Steal<rustc_index::vec::IndexVec<rustc_middle::mir::Promoted, rustc_middle::mir::Body>>)>::{closure#3}>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, (&rustc_data_structures::steal::Steal<rustc_middle::mir::Body>, &rustc_data_structures::steal::Steal<rustc_index::vec::IndexVec<rustc_middle::mir::Promoted, rustc_middle::mir::Body>>)>>
  38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
  39: rustc_borrowck::mir_borrowck
  40: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
  41: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>
  42: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::query::BorrowCheckResult, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>::{closure#3}>
  43: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>>
  44: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  45: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_interface::passes::analysis::{closure#2}::{closure#0}>
  46: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
  47: rustc_interface::passes::analysis
  48: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  49: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  50: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  51: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  52: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  53: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  54: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
  55: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
