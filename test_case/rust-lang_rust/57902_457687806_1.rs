
error: internal compiler error: src/librustc/ty/subst.rs:426: Region parameter out of range when substituting in region 's (root type=Some(&'s ())) (index=0)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:527:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
  15: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  16: rustc::traits::project::opt_normalize_projection_type
  17: rustc::traits::project::normalize_projection_type
  18: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  19: <smallvec::SmallVec<A> as core::iter::traits::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  20: rustc::ty::fold::TypeFoldable::fold_with
  21: rustc::ty::fold::TypeFoldable::fold_with
  22: rustc::ty::fold::TypeFoldable::fold_with
  23: rustc::traits::project::normalize
  24: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
  25: <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next
  26: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  27: rustc::ty::wf::trait_obligations
  28: rustc::ty::context::GlobalCtxt::enter_local
  29: rustc_typeck::check::wfcheck::check_item_well_formed
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  34: rustc::session::Session::track_errors
  35: rustc::util::common::time
  36: rustc_typeck::check_crate
  37: <std::thread::local::LocalKey<T>>::with
  38: rustc::ty::context::TyCtxt::create_and_enter
  39: rustc_driver::driver::compile_input
  40: <scoped_tls::ScopedKey<T>>::set
  41: rustc_driver::run_compiler
  42: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [check_item_well_formed] processing `<() as Iter>`
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (01f8e25b1 2019-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.
