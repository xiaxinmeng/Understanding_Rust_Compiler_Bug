
   Compiling playground v0.0.1 (/playground)
error[E0412]: cannot find type `n` in this scope
 --> src/main.rs:2:8
  |
2 |     2: n([u8; 3:fn()])
  |        ^ expecting a type here because of type ascription

error: internal compiler error: compiler/rustc_middle/src/ty/context.rs:2713:21: No bound vars found for "type fn() (hir_id=HirId { owner: DefId(0:3 ~ playground[7419]::main), local_id: 7 })" (HirId { owner: DefId(0:3 ~ playground[7419]::main), local_id: 7 })

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug
   3: <rustc_errors::Handler>::bug
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: <rustc_middle::ty::context::TyCtxt>::late_bound_vars
   8: <dyn rustc_typeck::astconv::AstConv>::ty_of_fn
   9: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner
  10: <rustc_typeck::check::fn_ctxt::FnCtxt>::to_ty_saving_user_provided_ty
  11: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  12: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  13: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
  14: rustc_typeck::check::typeck
  15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  17: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
  18: rustc_typeck::check::typeck_item_bodies
  19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
  20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
  21: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
  22: rustc_typeck::check_crate
  23: rustc_interface::passes::analysis
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  25: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  27: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  28: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  29: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  30: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
