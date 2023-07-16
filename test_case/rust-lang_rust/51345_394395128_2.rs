
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some((_3, bw0))`,
 right: `None`: never found an activation for this borrow!', librustc_mir\borrow_check\borrow_set.rs:121:9
stack backtrace:
   0: <std::sync::mpsc::RecvTimeoutError as core::fmt::Debug>::fmt
   1: <std::sys::windows::dynamic_lib::DynamicLibrary as core::ops::drop::Drop>::drop
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::middle::const_val::ErrKind<'a>>::lift_to_tcx
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: <rustc_mir::borrow_check::borrow_set::BorrowData<'tcx> as core::fmt::Display>::fmt
   8: rustc_mir::borrow_check::provide
   9: <rustc_mir::dataflow::move_paths::abs_domain::AbstractOperand as core::fmt::Debug>::fmt
  10: <rustc_mir::borrow_check::borrow_set::BorrowData<'tcx> as core::fmt::Debug>::fmt
  11: rustc_mir::borrow_check::provide
  12: rustc::ty::maps::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::maps::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  13: rustc::ty::context::tls::track_diagnostic
  14: rustc::ty::context::tls::track_diagnostic
  15: rustc::dep_graph::graph::DepGraph::assert_ignored
  16: rustc::ty::context::tls::track_diagnostic
  17: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read
  18: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read
  19: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  20: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  21: <humantime::duration::Error as std::error::Error>::cause
  22: <rustc_driver::pretty::NoAnn<'hir> as rustc_driver::pretty::HirPrinterSupport<'hir>>::sess
  23: <unknown>
  24: rustc_driver::driver::compile_input
  25: rustc_driver::run_compiler
  26: rustc_driver::target_features::add_configuration
  27: rustc_driver::target_features::add_configuration
  28: _rust_maybe_catch_panic
  29: rustc_driver::set_sigpipe_handler
  30: rustc_driver::main
  31: <unknown>
  32: std::panicking::update_panic_count
  33: _rust_maybe_catch_panic
  34: std::rt::lang_start_internal
  35: <unknown>
  36: <unknown>
  37: BaseThreadInitThunk
  38: RtlUserThreadStart
query stack during panic:
#0 [mir_borrowck] processing `main`
end of query stack
