
error: internal compiler error: src/librustc/ty/subst.rs:565: type parameter `T/#0` (T/0) out of range when substituting (root type=Some(T)) substs=[]

thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1063
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::span_bug
  14: rustc_errors::Handler::span_bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::span_bug_fmt
  20: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  21: rustc::ty::fold::TypeFoldable::fold_with
  22: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_const
  23: rustc::ty::normalize_erasing_regions::<impl rustc::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
  24: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
  25: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  26: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
  27: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
  28: rustc::ty::query::__query_compute::const_eval_raw
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  33: rustc::ty::query::__query_compute::const_eval_validated
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  38: rustc::ty::query::__query_compute::const_eval_validated
  39: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  40: rustc::dep_graph::graph::DepGraph::with_task_impl
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  42: rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_resolve
  43: rustc::ty::sty::Const::eval::{{closure}}
  44: rustc::ty::sty::Const::eval
  45: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  46: <rustc_infer::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  47: rustc_infer::traits::project::normalize
  48: rustc_infer::infer::InferCtxt::partially_normalize_associated_types_in
  49: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  50: rustc::ty::context::GlobalCtxt::enter_local
  51: rustc_typeck::check::wfcheck::check_item_well_formed
  52: rustc::ty::query::__query_compute::check_item_well_formed
  53: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_item_well_formed>::compute
  54: rustc::dep_graph::graph::DepGraph::with_task_impl
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  57: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  58: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  59: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  60: rustc_hir::hir::Crate::par_visit_all_item_likes
  61: rustc_session::session::Session::track_errors
  62: rustc_typeck::check_crate
  63: rustc_interface::passes::analysis
  64: rustc::ty::query::__query_compute::analysis
  65: rustc::dep_graph::graph::DepGraph::with_task_impl
  66: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  67: rustc::ty::context::tls::enter_global
  68: rustc_interface::interface::run_compiler_in_existing_thread_pool
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.1 (8d69840ab 2020-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [const_eval_raw] const-evaluating `Buffer::buf::{{constant}}#0`
#1 [const_eval_validated] const-evaluating + checking `Buffer::buf::{{constant}}#0`
#2 [const_eval_validated] const-evaluating + checking `Buffer::buf::{{constant}}#0`
#3 [check_item_well_formed] processing `Buffer`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
