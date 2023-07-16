
thread 'rustc' panicked at 'index out of bounds: the len is 8 but the index is 18446744073709551615', src/libcollections/vec.rs:1488
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: std::panicking::begin_panic_fmt
   6: rust_begin_unwind
   7: core::panicking::panic_fmt
   8: core::panicking::panic_bounds_check
   9: rustc::ty::AdtDef::discriminant_for_variant
  10: rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_rvalue
  11: rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block
  12: rustc_trans::mir::trans_mir
  13: rustc_trans::trans_item::TransItem::define
  14: rustc_trans::base::trans_crate
  15: rustc_driver::driver::phase_4_translate_to_llvm
  16: rustc_driver::driver::compile_input::{{closure}}
  17: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  18: rustc::ty::context::TyCtxt::create_and_enter
  19: rustc_driver::driver::phase_3_run_analysis_passes
  20: rustc_driver::driver::compile_input
  21: rustc_driver::run_compiler
  22: std::panicking::try::do_call
  23: __rust_maybe_catch_panic
  24: <F as alloc::boxed::FnBox<A>>::call_box
  25: std::sys::imp::thread::Thread::new::thread_start
  26: _pthread_body
  27: _pthread_start
