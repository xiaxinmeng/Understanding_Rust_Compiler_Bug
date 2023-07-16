
error: internal compiler error: src\librustc\hir\def.rs:166: attempted .def_id() on invalid def: Local(NodeId(20))

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:501:8
stack backtrace:
   0: <std::thread::ThreadId as core::fmt::Debug>::fmt
   1: <std::sync::condvar::Condvar as core::fmt::Debug>::fmt
   2: std::panicking::Location::column
   3: std::panicking::Location::column
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::snippet::Style as core::fmt::Debug>::fmt
   6: rustc_errors::Handler::bug
   7: rustc::ty::maps::<impl rustc::ty::maps::queries::fully_normalize_monormophic_ty<'tcx>>::try_get
   8: rustc::ty::context::tls::span_debug
   9: rustc::session::bug_fmt
  10: rustc::session::bug_fmt
  11: rustc::hir::def::Def::def_id
  12: <unknown>
  13: <unknown>
  14: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  15: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  16: rustc::hir::intravisit::NestedVisitorMap::inter
  17: rustc::lint::context::check_crate
  18: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  19: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  20: <rustc_driver::profile::trace::Query as core::fmt::Debug>::fmt
  21: rustc_driver::driver::compile_input
  22: rustc_driver::run_compiler
  23: <unknown>
  24: <unknown>
  25: _rust_maybe_catch_panic
  26: <unknown>
  27: <std::sync::condvar::Condvar as core::fmt::Debug>::fmt
  28: std::sys::windows::thread::Thread::new
  29: BaseThreadInitThunk
thread 'main' panicked at 'rustc_thread failed: Any', src\libcore\result.rs:906:4
stack backtrace:
   0: <std::thread::ThreadId as core::fmt::Debug>::fmt
   1: <std::sync::condvar::Condvar as core::fmt::Debug>::fmt
   2: std::panicking::Location::column
   3: std::panicking::Location::column
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic_fmt
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: <unknown>
  10: <unknown>
  11: _rust_maybe_catch_panic
  12: std::rt::lang_start
  13: <unknown>
  14: BaseThreadInitThunk
error: Could not compile `repro`.
