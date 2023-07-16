
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:43:32: could not fully normalize `Const { ty: fn() -> usize {std::mem::size_of::<<Self as HardcodedPayload>::ArrayType>}, val: Value(Scalar(<ZST>)) }`

thread 'rustc' panicked at 'Box<Any>', /rustc/5fa22fe6f821ac3801d05f624b123dda25fde32c/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_infer::infer::InferCtxtBuilder::enter
   8: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  10: rustc_data_structures::stack::ensure_sufficient_stack
  11: rustc_query_system::query::plumbing::force_query_with_job
  12: rustc_query_system::query::plumbing::get_query_impl
  13: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
  14: rustc_middle::ty::instance::Instance::subst_mir_and_normalize_erasing_regions
  15: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
  16: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
  17: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_allocation_raw>::compute
  18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  19: rustc_data_structures::stack::ensure_sufficient_stack
  20: rustc_query_system::query::plumbing::force_query_with_job
  21: rustc_query_system::query::plumbing::get_query_impl
  22: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
  23: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute
  24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  25: rustc_data_structures::stack::ensure_sufficient_stack
  26: rustc_query_system::query::plumbing::force_query_with_job
  27: rustc_query_system::query::plumbing::get_query_impl
  28: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
  29: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute
  30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  31: rustc_data_structures::stack::ensure_sufficient_stack
  32: rustc_query_system::query::plumbing::force_query_with_job
  33: rustc_query_system::query::plumbing::get_query_impl
  34: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  35: rustc_middle::ty::consts::Const::try_eval_usize
  36: rustc_middle::ty::error::<impl rustc_middle::ty::TyS>::sort_string
  37: <rustc_middle::ty::error::TypeError as core::fmt::Display>::fmt
  38: core::fmt::write
             at /rustc/5fa22fe6f821ac3801d05f624b123dda25fde32c/library/core/src/fmt/mod.rs:1096:17
  39: core::fmt::Write::write_fmt
  40: rustc_infer::infer::error_reporting::<impl rustc_infer::infer::InferCtxt>::note_type_err
  41: rustc_infer::infer::error_reporting::<impl rustc_infer::infer::InferCtxt>::report_and_explain_type_error
  42: rustc_infer::infer::InferCtxt::report_mismatched_types
  43: rustc_typeck::check::demand::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  44: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_overloaded_binop
  45: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_binop
  46: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  48: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  49: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  50: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  51: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  52: rustc_typeck::check::check::check_fn
  53: rustc_infer::infer::InferCtxtBuilder::enter
  54: rustc_typeck::check::typeck
  55: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck>::compute
  56: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  57: rustc_data_structures::stack::ensure_sufficient_stack
  58: rustc_query_system::query::plumbing::force_query_with_job
  59: rustc_query_system::query::plumbing::get_query_impl
  60: rustc_query_system::query::plumbing::ensure_query_impl
  61: rustc_typeck::check::typeck_item_bodies
  62: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
  63: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  64: rustc_data_structures::stack::ensure_sufficient_stack
  65: rustc_query_system::query::plumbing::force_query_with_job
  66: rustc_query_system::query::plumbing::get_query_impl
  67: rustc_typeck::check_crate
  68: rustc_interface::passes::analysis
  69: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  70: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  71: rustc_data_structures::stack::ensure_sufficient_stack
  72: rustc_query_system::query::plumbing::force_query_with_job
  73: rustc_query_system::query::plumbing::get_query_impl
  74: rustc_interface::passes::QueryContext::enter
  75: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  76: rustc_span::with_source_map
  77: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (5fa22fe6f 2021-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `std::mem::size_of::<<Self as HardcodedPayload>::ArrayType>`
#1 [eval_to_allocation_raw] const-evaluating + checking `HardcodedPayload::test::{constant#0}`
#2 [eval_to_const_value_raw] simplifying constant for the type system `HardcodedPayload::test::{constant#0}`
#3 [eval_to_const_value_raw] simplifying constant for the type system `HardcodedPayload::test::{constant#0}`
#4 [typeck] type-checking `HardcodedPayload::test`
#5 [typeck_item_bodies] type-checking all item bodies
#6 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `playground`

To learn more, run the command again with --verbose.
