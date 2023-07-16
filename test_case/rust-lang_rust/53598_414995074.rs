`

   Compiling issue_53598 v0.1.0 (file:///tmp/issue_53598)
error: internal compiler error: librustc/ty/subst.rs:479: Type parameter `C/#0` (C/0) out of range when substituting (root type=Some(Node<HtmlEngine, Div, HtmlEngine, std::option::Option<C>>)) substs=[]
thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:517:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:479
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::span_bug_fmt
  14: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  15: rustc::ty::fold::TypeFoldable::fold_with
  16: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  17: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  18: rustc::ty::fold::TypeFoldable::fold_with
  19: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  20: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  21: rustc::infer::InferCtxt::commit_if_ok
  22: <rustc::traits::query::type_op::custom::CustomTypeOp<F, G> as rustc::traits::query::type_op::TypeOp<'gcx, 'tcx>>::fully_perform
  23: rustc_mir::borrow_check::nll::type_check::type_check
  24: rustc_mir::borrow_check::nll::compute_regions
  25: rustc_mir::borrow_check::do_mir_borrowck
  26: rustc::ty::context::GlobalCtxt::enter_local
  27: rustc_mir::borrow_check::mir_borrowck
  28: rustc::ty::query::__query_compute::mir_borrowck
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  30: rustc::ty::context::tls::with_context
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  35: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  36: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  37: rustc::util::common::time
  38: rustc::ty::context::tls::enter_context
  39: <std::thread::local::LocalKey<T>>::with
  40: rustc::ty::context::TyCtxt::create_and_enter
  41: rustc_driver::driver::compile_input
  42: rustc_driver::run_compiler_with_pool
  43: syntax::with_globals
  44: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  45: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  46: rustc_driver::run
  47: rustc_driver::main
  48: std::rt::lang_start::{{closure}}
  49: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  50: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  51: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  52: main
  53: __libc_start_main
  54: <unknown>
query stack during panic:
#0 [mir_borrowck] processing `<Div as View<HtmlEngine, HtmlEngine>>::build`
end of query stack
error: aborting due to previous error
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.30.0-nightly (33b923fd4 2018-08-18) running on x86_64-unknown-linux-gnu
note: compiler flags: -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin
note: some of the compiler flags provided by cargo are hidden
error: Could not compile `issue_53598`.
To learn more, run the command again with --verbose.
