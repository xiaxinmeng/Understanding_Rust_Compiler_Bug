
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', libcore\slice\mod.rs:2103:10
stack backtrace:
   0: <std::sync::mpsc::RecvTimeoutError as core::fmt::Debug>::fmt
   1: <std::path::Iter<'a> as core::convert::AsRef<std::path::Path>>::as_ref
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::ty::adjustment::AutoBorrow<'a>>::lift_to_tcx
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::panicking::panic_bounds_check
  10: <rustc_lint::nonstandard_style::NonUpperCaseGlobals as rustc::lint::LateLintPass<'a, 'tcx>>::check_pat
  11: <rustc_lint::builtin::TypeAliasBounds as rustc::lint::LateLintPass<'a, 'tcx>>::check_item
  12: <rustc_lint::register_builtins::BuiltinCombinedLateLintPass as rustc::lint::LateLintPass<'a, 'tcx>>::check_item
  13: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  14: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  15: rustc::hir::intravisit::NestedVisitorMap::inter
  16: rustc::lint::context::check_crate
  17: <humantime::date::Error as std::error::Error>::cause
  18: rustc_driver::target_features::add_configuration
  19: rustc_driver::profile::dump
  20: rustc_driver::driver::compile_input
  21: rustc_driver::run_compiler
  22: <rustc_driver::profile::trace::Query as core::fmt::Debug>::fmt
  23: rustc_driver::run_compiler
  24: <rustc_driver::profile::trace::Query as core::fmt::Debug>::fmt
  25: _rust_maybe_catch_panic
  26: rustc_driver::profile::dump
  27: rustc_driver::main
  28: <unknown>
  29: std::panicking::update_panic_count
  30: _rust_maybe_catch_panic
  31: std::rt::lang_start_internal
  32: <unknown>
  33: <unknown>
  34: BaseThreadInitThunk
  35: RtlUserThreadStart
query stack during panic:
end of query stack
