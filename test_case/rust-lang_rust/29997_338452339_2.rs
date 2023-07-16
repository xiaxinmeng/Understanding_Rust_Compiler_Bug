 
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:437:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: rustc_errors::Handler::span_bug
   6: rustc::session::opt_span_bug_fmt::{{closure}}
   7: rustc::session::opt_span_bug_fmt
   8: rustc::session::span_bug_fmt
   9: rustc::traits::trans::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>>::trans_fulfill_obligation::{{closure}}
  10: rustc::traits::trans::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>>::trans_fulfill_obligation
  11: rustc_trans::monomorphize::resolve
  12: <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  13: rustc::mir::visit::Visitor::visit_mir
  14: rustc_trans::collector::collect_items_rec
  15: rustc_trans::collector::collect_items_rec
  16: rustc_trans::base::collect_and_partition_translation_items::{{closure}}
  17: rustc_trans::base::trans_crate
  18: rustc_driver::driver::phase_4_translate_to_llvm
  19: rustc_driver::driver::compile_input::{{closure}}
  20: rustc::ty::context::TyCtxt::create_and_enter
  21: rustc_driver::driver::compile_input
  22: rustc_driver::run_compiler
