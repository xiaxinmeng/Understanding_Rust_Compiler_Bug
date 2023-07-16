console
error[E0412]: cannot find type `Thing` in this scope
  --> src/main.rs:14:23
   |
14 |     let thing = get::<Thing>();
   |                       ^^^^^ not found in this scope

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:853:17: ErrorReporting Overflow should not reach `report_selection_err` call

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
   8: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
   9: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  10: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
  11: <rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  12: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  13: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  14: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  15: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
  16: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  17: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  18: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  19: rustc_typeck::check::check::check_fn
  20: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
  21: rustc_typeck::check::typeck
  22: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
  23: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  26: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
  27: rustc_typeck::check::typeck_item_bodies
  28: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), ()>
  29: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>
  30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
  31: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
  32: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
  33: rustc_typeck::check_crate
  34: rustc_interface::passes::analysis
  35: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  36: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  39: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  40: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  41: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  42: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  43: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65c55bf93 2021-11-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0412`.
