
error: internal compiler error: src/librustc/ty/subst.rs:546: Type parameter `T/#0` (T/0) out of range when substituting (root type=Some(fn() -> usize {std::mem::size_of::<&T>})) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:570:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::span_bug_fmt
  14: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  16: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  17: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  18: rustc::ty::fold::TypeFoldable::fold_with
  19: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  20: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  21: rustc_mir::interpret::eval_context::InterpretCx<M>::monomorphize
  22: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpretCx<M>>::eval_const_to_op
  23: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpretCx<M>>::run
  24: rustc_mir::const_eval::eval_body_using_ecx
  25: rustc_mir::const_eval::const_eval_raw_provider
  26: rustc::ty::query::__query_compute::const_eval_raw
  27: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  30: rustc_mir::const_eval::const_eval_provider
  31: rustc::ty::query::__query_compute::const_eval
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc_typeck::check::FnCtxt::check_expr_kind
  36: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  37: rustc_typeck::check::FnCtxt::check_decl_initializer
  38: rustc_typeck::check::FnCtxt::check_decl_local
  39: rustc_typeck::check::FnCtxt::check_stmt
  40: rustc_typeck::check::FnCtxt::check_block_with_expected
  41: rustc_typeck::check::FnCtxt::check_expr_kind
  42: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  43: rustc_typeck::check::FnCtxt::check_return_expr
  44: rustc_typeck::check::check_fn
  45: rustc::ty::context::GlobalCtxt::enter_local
  46: rustc_typeck::check::typeck_tables_of
  47: rustc::ty::query::__query_compute::typeck_tables_of
  48: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  49: rustc::dep_graph::graph::DepGraph::with_task_impl
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  51: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  52: rustc_typeck::check::typeck_item_bodies
  53: rustc::ty::query::__query_compute::typeck_item_bodies
  54: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute
  55: rustc::dep_graph::graph::DepGraph::with_task_impl
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  57: rustc::util::common::time
  58: rustc_typeck::check_crate
  59: rustc_interface::passes::analysis
  60: rustc::ty::query::__query_compute::analysis
  61: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  62: rustc::dep_graph::graph::DepGraph::with_task_impl
  63: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  64: rustc::ty::context::tls::enter_global
  65: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  66: rustc_interface::passes::create_global_ctxt::{{closure}}
  67: rustc_interface::passes::BoxedGlobalCtxt::enter
  68: rustc_interface::interface::run_compiler_in_existing_thread_pool
  69: std::thread::local::LocalKey<T>::with
  70: scoped_tls::ScopedKey<T>::set
  71: syntax::with_globals
query stack during panic:
#0 [const_eval_raw] const-evaluating `foo::{{constant}}#0`
#1 [const_eval] const-evaluating + checking `foo::{{constant}}#0`
#2 [typeck_tables_of] processing `foo`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.35.0-nightly (2975a3c4b 2019-04-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

