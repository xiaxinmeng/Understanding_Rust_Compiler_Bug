
   Compiling foo v0.1.0 (file:///tmp/tmp.s3wncRLBaT/foo)
error: internal compiler error: librustc/ty/subst.rs:425: Region parameter out of range when substituting in region 'a (root type=Some(std::fmt::Display + 'a)) (index=1)

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:499:9
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
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::span_bug_fmt
  14: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  17: rustc::traits::project::opt_normalize_projection_type
  18: rustc::traits::project::normalize_projection_type
  19: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  20: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  21: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  22: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  23: rustc::ty::fold::TypeFoldable::fold_with
  24: rustc::traits::project::normalize
  25: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  26: rustc::ty::context::tls::with_related_context
  27: rustc::infer::InferCtxtBuilder::enter
  28: rustc_typeck::check::wfcheck::check_associated_item
  29: rustc_typeck::check::wfcheck::check_impl_item
  30: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::check_impl_item_well_formed<'tcx>>::compute
  31: rustc::ty::context::tls::with_context
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  37: rustc::hir::Crate::visit_all_item_likes
  38: rustc_typeck::check::check_wf_new
  39: rustc::util::common::time
  40: rustc_typeck::check_crate
  41: rustc::ty::context::tls::enter_context
  42: <std::thread::local::LocalKey<T>>::with
  43: rustc::ty::context::TyCtxt::create_and_enter
  44: rustc_driver::driver::compile_input
  45: rustc_driver::run_compiler_with_pool
  46: syntax::with_globals
  47: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  48: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  49: rustc_driver::run
  50: rustc_driver::main
  51: std::rt::lang_start::{{closure}}
  52: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  53: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  54: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  55: main
  56: __libc_start_main
  57: <unknown>
query stack during panic:
#0 [check_impl_item_well_formed] processing `<Wrap<T> as std::ops::Index<usize>>::index`
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (5bf68db6e 2018-05-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `foo`.

To learn more, run the command again with --verbose.
