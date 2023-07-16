
error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:42: could not fully normalize `fn() -> usize {std::mem::size_of::<<T as datatypes::ArrowPrimitiveType>::Native>}`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:912:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc::ty::context::GlobalCtxt::enter_local
  22: rustc_traits::normalize_erasing_regions::normalize_ty_after_erasing_regions
  23: rustc::ty::query::__query_compute::normalize_ty_after_erasing_regions
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::normalize_ty_after_erasing_regions>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc::traits::codegen::<impl rustc::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
  28: rustc::ty::instance::Instance::ty
  29: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_fn_call
  30: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
  31: rustc_mir::const_eval::const_eval_raw_provider
  32: rustc::ty::query::__query_compute::const_eval_raw
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  36: rustc_mir::interpret::eval_context::InterpCx<M>::const_eval_raw
  37: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_static_to_mplace
  38: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place
  39: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  40: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  41: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  42: rustc_mir::transform::run_passes
  43: rustc_mir::transform::run_optimization_passes
  44: rustc_mir::transform::optimized_mir
  45: rustc::ty::query::__query_compute::optimized_mir
  46: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc_metadata::encoder::EncodeContext::encode_optimized_mir
  50: <rustc_metadata::encoder::EncodeContext as rustc::hir::intravisit::Visitor>::visit_item
  51: rustc::hir::Crate::visit_all_item_likes
  52: rustc_metadata::encoder::EncodeContext::encode_crate_root
  53: rustc::dep_graph::graph::DepGraph::with_ignore
  54: rustc_metadata::encoder::encode_metadata
  55: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  56: rustc::ty::context::TyCtxt::encode_metadata
  57: rustc_interface::passes::encode_and_write_metadata
  58: rustc::util::common::time
  59: rustc_interface::passes::start_codegen
  60: rustc::ty::context::tls::enter_global
  61: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  62: rustc_interface::passes::create_global_ctxt::{{closure}}
  63: rustc_interface::queries::Query<T>::compute
  64: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  65: rustc_interface::interface::run_compiler_in_existing_thread_pool
  66: std::thread::local::LocalKey<T>::with
  67: scoped_tls::ScopedKey<T>::set
  68: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (8431f261d 2019-09-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: fn() -> usize {std::mem::size_of::<<T as datatypes::ArrowPrimitiveType>::Native>} }`
#1 [const_eval_raw] const-evaluating `tensor::Tensor::<'a, T>::new`
#2 [optimized_mir] processing `tensor::Tensor::<'a, T>::new`
end of query stack
error: aborting due to previous error
