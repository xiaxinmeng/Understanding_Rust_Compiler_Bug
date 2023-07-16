
   Compiling compiler_crash v0.1.0 (/tmp/compiler_crash)
error: internal compiler error: compiler/rustc_middle/src/ty/context.rs:703:13: node_type: no type for node `expr {
                                        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
                                        res
                                    } (hir_id=HirId { owner: DefId(0:3 ~ compiler_crash[58a7]::main), local_id: 41 })`

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/897e37553bba8b42751c67658967889d11ecd120/compiler/rustc_errors/src/lib.rs:1462:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug::<&alloc::string::String>
   3: <rustc_errors::Handler>::bug::<&alloc::string::String>
   4: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_opt<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_middle::ty::context::TypeckResults>::expr_ty_adjusted
   8: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code::<rustc_middle::ty::Predicate>
   9: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::note_obligation_cause
  10: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
  11: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  12: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  13: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  14: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  15: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  16: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  17: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  18: rustc_typeck::check::check::check_fn
  19: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
  20: rustc_typeck::check::typeck
  21: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
  22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  24: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  25: rustc_typeck::check::typeck_item_bodies
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), ()>
  27: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
  28: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
  29: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
  30: rustc_typeck::check_crate
  31: rustc_interface::passes::analysis
  32: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  33: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  34: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  35: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  36: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  37: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0 (897e37553 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `compiler_crash`
