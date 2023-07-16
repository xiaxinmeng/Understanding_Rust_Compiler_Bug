
thread 'rustc' panicked at 'failed to create span for type arguments', libcore/option.rs:1000:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
             at libstd/panicking.rs:481
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:391
   7: rust_begin_unwind
             at libstd/panicking.rs:326
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::option::expect_failed
             at libcore/option.rs:1000
  10: <clippy_lints::types::ImplicitHasher as rustc::lint::LateLintPass<'a, 'tcx>>::check_item
  11: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  12: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  13: rustc::hir::intravisit::walk_crate
  14: rustc::lint::context::check_crate
  15: rustc::util::common::time
  16: rustc::ty::context::tls::enter_context
  17: <std::thread::local::LocalKey<T>>::with
  18: rustc::ty::context::TyCtxt::create_and_enter
  19: rustc_driver::driver::compile_input
  20: rustc_driver::run_compiler_with_pool
  21: <scoped_tls::ScopedKey<T>>::set
  22: rustc_driver::run_compiler
  23: <scoped_tls::ScopedKey<T>>::set
