
❯ RUST_BACKTRACE=1 cargo +nightly c
    Checking rustc-ice-example v0.1.0 (/Users/wm/Developer/rustc-ice-example)
error[E0432]: unresolved import `sqlx::PgConnection`
 --> src/lib.rs:3:5
  |
3 | use sqlx::PgConnection;
  |     ^^^^^^------------
  |     |     |
  |     |     help: a similar name exists in the module: `Connection`
  |     no `PgConnection` in the root

error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:2150:18: tuple_fields called on non-tuple

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt
   6: rustc_middle::util::bug::opt_span_bug_fmt
   7: rustc_middle::util::bug::bug_fmt
   8: rustc_middle::ty::sty::<impl rustc_middle::ty::TyS>::tuple_fields
   9: <rustc_trait_selection::opaque_types::ConstrainOpaqueTypeRegionVisitor<OP> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  10: rustc_middle::ty::fold::TypeFoldable::visit_with
  11: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
  12: <rustc_trait_selection::opaque_types::ConstrainOpaqueTypeRegionVisitor<OP> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  13: <rustc_infer::infer::InferCtxt as rustc_trait_selection::opaque_types::InferCtxtExt>::constrain_opaque_types
  14: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  15: rustc_hir::intravisit::walk_expr
  16: <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr
  17: rustc_hir::intravisit::walk_expr
  18: <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr
  19: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  20: rustc_typeck::check::regionck::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::regionck_fn
  21: rustc_infer::infer::InferCtxtBuilder::enter
  22: rustc_typeck::check::typeck
  23: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  25: rustc_data_structures::stack::ensure_sufficient_stack
  26: rustc_query_system::query::plumbing::get_query_impl
  27: rustc_middle::ty::context::TyCtxt::typeck_opt_const_arg
  28: rustc_mir_build::thir::cx::Cx::new
  29: rustc_infer::infer::InferCtxtBuilder::enter
  30: rustc_mir_build::build::mir_built
  31: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
  32: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  33: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  34: rustc_data_structures::stack::ensure_sufficient_stack
  35: rustc_query_system::query::plumbing::get_query_impl
  36: rustc_mir::transform::check_unsafety::unsafety_check_result
  37: core::ops::function::FnOnce::call_once
  38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::unsafety_check_result>::compute
  39: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  40: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  41: rustc_data_structures::stack::ensure_sufficient_stack
  42: rustc_query_system::query::plumbing::get_query_impl
  43: rustc_query_system::query::plumbing::ensure_query_impl
  44: rustc_mir::transform::mir_const
  45: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_const>::compute
  46: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  47: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  48: rustc_data_structures::stack::ensure_sufficient_stack
  49: rustc_query_system::query::plumbing::get_query_impl
  50: rustc_mir::transform::mir_promoted
  51: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
  52: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  54: rustc_data_structures::stack::ensure_sufficient_stack
  55: rustc_query_system::query::plumbing::get_query_impl
  56: rustc_mir::borrow_check::mir_borrowck
  57: core::ops::function::FnOnce::call_once
  58: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  59: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  60: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  61: rustc_data_structures::stack::ensure_sufficient_stack
  62: rustc_query_system::query::plumbing::get_query_impl
  63: rustc_typeck::collect::type_of::type_of
  64: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::type_of>::compute
  65: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  66: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  67: rustc_data_structures::stack::ensure_sufficient_stack
  68: rustc_query_system::query::plumbing::get_query_impl
  69: rustc_typeck::check::check::check_item_type
  70: rustc_middle::hir::map::Map::visit_item_likes_in_module
  71: rustc_typeck::check::check::check_mod_item_types
  72: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_item_types>::compute
  73: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  75: rustc_data_structures::stack::ensure_sufficient_stack
  76: rustc_query_system::query::plumbing::get_query_impl
  77: rustc_query_system::query::plumbing::ensure_query_impl
  78: rustc_session::utils::<impl rustc_session::session::Session>::time
  79: rustc_typeck::check_crate
  80: rustc_interface::passes::analysis
  81: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  82: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  83: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  84: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
  85: rustc_query_system::query::plumbing::get_query_impl
  86: rustc_interface::passes::QueryContext::enter
  87: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  88: rustc_span::with_source_map
  89: rustc_interface::interface::create_compiler_and_run
  90: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=/usr/local/bin/zld --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `minimal_example`
#1 [mir_built] building MIR for `minimal_example`
#2 [unsafety_check_result] unsafety-checking `minimal_example`
#3 [mir_const] processing MIR for `minimal_example`
#4 [mir_promoted] processing `minimal_example`
#5 [mir_borrowck] borrow-checking `minimal_example`
#6 [type_of] computing type of `minimal_example::{opaque#0}`
#7 [check_mod_item_types] checking item types in top-level module
#8 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc-ice-example`

To learn more, run the command again with --verbose.