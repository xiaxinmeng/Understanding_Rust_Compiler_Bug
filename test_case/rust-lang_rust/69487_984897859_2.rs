
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:16
  |
2 |     [9; || [9; []]];
  |                ^^ expected `usize`, found array of 0 elements
  |
  = note: expected type `usize`
            found array `[_; 0]`

error: internal compiler error: compiler/rustc_const_eval/src/interpret/eval_context.rs:198:17: The type checker should prevent reading from a never-written local

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1171:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
   8: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
   9: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
  11: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, core::result::Result<rustc_middle::mir::interpret::value::ConstValue, rustc_middle::mir::interpret::error::ErrorHandled>>>
  13: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  14: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  15: <rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  16: <rustc_infer::infer::InferCtxt>::const_eval_resolve
  17: rustc_trait_selection::traits::const_evaluatable::is_const_evaluatable
  18: <rustc_trait_selection::traits::fulfill::FulfillProcessor>::progress_changed_obligations
  19: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
  20: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_with_constness_where_possible
  21: <rustc_typeck::check::fn_ctxt::FnCtxt>::type_inference_fallback
  22: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
  23: rustc_typeck::check::typeck
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  26: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
  27: rustc_typeck::check::typeck_item_bodies
  28: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
  29: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
  30: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
  31: rustc_typeck::check_crate
  32: rustc_interface::passes::analysis
  33: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  34: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  35: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  36: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  37: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  38: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (48a5999fc 2021-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#1 [eval_to_const_value_raw] simplifying constant for the type system `main::{constant#0}`
#2 [typeck] type-checking `main`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
