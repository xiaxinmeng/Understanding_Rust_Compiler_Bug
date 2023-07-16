text
error: internal compiler error: src/librustc/ty/subst.rs:426: Region parameter out of range when substituting in region 'a (root type=Some(&'a Bar)) (index=0)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:558:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::span_bug_fmt
  14: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  17: rustc::traits::project::opt_normalize_projection_type
  18: rustc::traits::project::normalize_projection_type
  19: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  20: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  21: rustc::ty::fold::TypeFoldable::fold_with
  22: rustc::ty::fold::TypeFoldable::fold_with
  23: rustc::ty::fold::TypeFoldable::fold_with
  24: rustc::traits::project::normalize
  25: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
  26: <core::iter::adapters::flatten::FlatMap<I, U, F> as core::iter::traits::iterator::Iterator>::next
  27: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  28: rustc::ty::wf::trait_obligations
  29: rustc::ty::context::GlobalCtxt::enter_local
  30: rustc_typeck::check::wfcheck::check_item_well_formed
  31: rustc::ty::query::__query_compute::check_item_well_formed
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  35: rustc::hir::Crate::visit_all_item_likes
  36: rustc::util::common::time
  37: rustc_typeck::check_crate
  38: <std::thread::local::LocalKey<T>>::with
  39: rustc::ty::context::TyCtxt::create_and_enter
  40: rustc_driver::driver::compile_input
  41: rustc_driver::run_compiler_with_pool
  42: <scoped_tls::ScopedKey<T>>::set
  43: rustc_driver::run_compiler
  44: syntax::with_globals
  45: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  46: <F as alloc::boxed::FnBox<A>>::call_box
  47: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/6c2484dc3c532c052f159264e970278d8b77cdc9/src/liballoc/boxed.rs:759
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:81
  48: start_thread
  49: __clone
query stack during panic:
#0 [check_item_well_formed] processing `<Foo as FooBar>`
end of query stack
error: aborting due to 3 previous errors

Some errors occurred: E0412, E0658.
For more information about an error, try `rustc --explain E0412`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.2 (6c2484dc3 2019-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
