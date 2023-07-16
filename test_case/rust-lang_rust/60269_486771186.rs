
 Documenting tower-hyper v0.1.0 (/Users/lucio/code/tower-hyper)
thread 'rustc' panicked at 'Infer', src/librustdoc/clean/mod.rs:2903:30
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: <&'tcx rustc::ty::TyS<'tcx> as rustdoc::clean::Clean<rustdoc::clean::Type>>::clean
   7: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
   8: rustdoc::clean::external_path
   9: <&'tcx rustc::ty::TyS<'tcx> as rustdoc::clean::Clean<rustdoc::clean::Type>>::clean
  10: <rustc::ty::Predicate<'a> as rustdoc::clean::Clean<core::option::Option<rustdoc::clean::WherePredicate>>>::clean
  11: rustdoc::clean::auto_trait::AutoTraitFinder::param_env_to_generics
  12: rustc::ty::context::tls::with_context::{{closure}}
  13: rustc::ty::context::GlobalCtxt::enter_local
  14: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics
  15: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impl_for
  16: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impls
  17: rustdoc::clean::def_ctor::get_def_from_node_id
  18: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx> as rustdoc::fold::DocFolder>::fold_item
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  20: rustdoc::fold::DocFolder::fold_inner_recur
  21: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx> as rustdoc::fold::DocFolder>::fold_item
  22: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  23: rustdoc::fold::DocFolder::fold_inner_recur
  24: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx> as rustdoc::fold::DocFolder>::fold_item
  25: rustdoc::passes::collect_trait_impls::collect_trait_impls
  26: rustdoc::core::run_core::{{closure}}::{{closure}}
  27: <std::thread::local::LocalKey<T>>::with
  28: rustc::ty::context::TyCtxt::create_and_enter
  29: rustc_driver::driver::phase_3_run_analysis_passes
  30: <scoped_tls::ScopedKey<T>>::set
  31: <scoped_tls::ScopedKey<T>>::set
  32: syntax::with_globals

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0 (91856ed52 2019-04-10) running on x86_64-apple-darwin

error: Could not document `tower-hyper`.

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-name tower_hyper src/lib.rs --color always -o /Users/lucio/code/tower-hyper/target/doc -L dependency=/Users/lucio/code/tower-hyper/target/debug/deps --extern futures=/Users/lucio/code/tower-hyper/target/debug/deps/libfutures-1e2999450b2419eb.rmeta --extern http=/Users/lucio/code/tower-hyper/target/debug/deps/libhttp-cf0a3164f3f563fd.rmeta --extern hyper=/Users/lucio/code/tower-hyper/target/debug/deps/libhyper-452ca54e3f820463.rmeta --extern log=/Users/lucio/code/tower-hyper/target/debug/deps/liblog-75a074a4b89524ff.rmeta --extern tokio_buf=/Users/lucio/code/tower-hyper/target/debug/deps/libtokio_buf-cd9601f408caeb33.rmeta --extern tokio_executor=/Users/lucio/code/tower-hyper/target/debug/deps/libtokio_executor-c67f7b74fd64079c.rmeta --extern tokio_io=/Users/lucio/code/tower-hyper/target/debug/deps/libtokio_io-1de1b000031540d9.rmeta --extern tower=/Users/lucio/code/tower-hyper/target/debug/deps/libtower-dfc827168ff91516.rmeta --extern tower_http=/Users/lucio/code/tower-hyper/target/debug/deps/libtower_http-b4824a8d9ae1e7e3.rmeta --extern tower_service=/Users/lucio/code/tower-hyper/target/debug/deps/libtower_service-167cf579d007ab89.rmeta` (exit code: 1)
