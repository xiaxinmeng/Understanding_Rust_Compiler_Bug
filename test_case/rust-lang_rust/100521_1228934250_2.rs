
thread 'rustc' panicked at 'Failed to extract DefId: local_def_id_to_hir_id ac4cfdf8347ee6ff-87dd7d1e09d48c0a', compiler/rustc_middle/src/dep_graph/dep_node.rs:273:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:142:14
   2: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id::{closure#0}
   3: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
   4: rustc_query_impl::query_callbacks::local_def_id_to_hir_id::force_from_dep_node
   5: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
   6: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   7: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   8: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   9: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  12: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  13: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
  15: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType>>, core::result::Result<&rustc_middle::infer::canonical::Canonical<rustc_middle::infer::canonical::QueryResponse<()>>, rustc_middle::traits::query::NoSolution>>
  16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType>>, core::result::Result<&rustc_middle::infer::canonical::Canonical<rustc_middle::infer::canonical::QueryResponse<()>>, rustc_middle::traits::query::NoSolution>>>
  17: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::type_op_ascribe_user_type, rustc_query_impl::plumbing::QueryCtxt>
  18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_op_ascribe_user_type
  19: <rustc_middle::traits::query::type_op::AscribeUserType as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::perform_query
  20: <rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
  21: <rustc_borrowck::type_check::TypeChecker>::fully_perform_op::<(), rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType>>
  22: rustc_borrowck::type_check::type_check
  23: rustc_borrowck::nll::compute_regions
  24: rustc_borrowck::do_mir_borrowck
  25: rustc_borrowck::mir_borrowck
  26: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
  27: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>
  28: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>>
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  30: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_interface::passes::analysis::{closure#2}::{closure#0}>
  31: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
  32: rustc_interface::passes::analysis
  33: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  35: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  36: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  37: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  38: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  39: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0 (4b91a6ea7 2022-08-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_op_ascribe_user_type] evaluating `type_op_ascribe_user_type` `Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: AscribeUserType { mir_ty: for<'r> fn(diesel::query_builder::update_statement::UpdateStatement<schema::table, diesel::query_builder::where_clause::WhereClause<diesel::expression::operators::Eq<schema::id, diesel::expression::bound::Bound<diesel::sql_types::Integer, &'r i32>>>, diesel::query_builder::update_statement::changeset::Assign<schema::id, diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>>>, &'r diesel::pg::connection::PgConnection) -> core::result::Result<Version, diesel::result::Error> {<diesel::query_builder::update_statement::UpdateStatement<schema::table, diesel::query_builder::where_clause::WhereClause<diesel::expression::operators::Eq<schema::id, diesel::expression::bound::Bound<diesel::sql_types::Integer, &i32>>>, diesel::query_builder::update_statement::changeset::Assign<schema::id, diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>>> as diesel::query_dsl::RunQueryDsl<diesel::pg::connection::PgConnection>>::get_result::<Version>}, def_id: DefId(20:4076 ~ diesel[c359]::query_dsl::RunQueryDsl::get_result), user_substs: UserSubsts { substs: [^1, ^2, Version], user_self_ty: None } } } }`
#1 [mir_borrowck] borrow-checking `build`
#2 [analysis] running analysis passes on this crate
end of query stack
warning: `cargo-registry` (lib) generated 5 warnings
error: could not compile `cargo-registry`; 5 warnings emitted
