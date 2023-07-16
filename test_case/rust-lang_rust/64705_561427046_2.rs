
error: internal compiler error: src\librustc_mir\monomorphize\collector.rs:804: cannot create local mono-item for DefId(14:8 ~ stuff[8787]::state[0]::{{impl}}[0]::new[0])

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:892:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSlice as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: <rustc_errors::snippet::Style as core::fmt::Debug>::fmt
   8: rustc_errors::HandlerInner::abort_if_errors_and_should_abort
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::bug_fmt
  11: rustc::ty::util::provide
  12: rustc::ty::util::provide
  13: rustc::util::bug::bug_fmt
  14: rustc::util::bug::bug_fmt
  15: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_place_base
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_place_base
  17: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  18: rustc_mir::monomorphize::collector::collect_crate_mono_items
  19: <rustc_mir::transform::instcombine::OptimizationFinder as rustc::mir::visit::Visitor>::visit_rvalue
  20: <rustc_mir::const_eval::ConstEvalError as core::fmt::Debug>::fmt
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items
  22: <rustc_mir::const_eval::ConstEvalError as core::fmt::Debug>::fmt
  23: rustc_mir::monomorphize::partitioning::compute_codegen_unit_name
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  25: <rustc_codegen_llvm::builder::Builder as core::ops::drop::Drop>::drop
  26: <rustc_codegen_llvm::debuginfo::metadata::MemberDescription as core::fmt::Debug>::fmt
  27: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  28: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  29: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  30: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  31: rustc_interface::passes::BoxedResolver::complete
  32: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  33: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  34: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  36: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  37: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  38: <rustc::ty::Predicate as rustc::ty::codec::EncodableWithShorthand>::variant
  39: rustc_driver::pretty::print_after_hir_lowering
  40: <rustc::ty::Predicate as rustc::ty::codec::EncodableWithShorthand>::variant
  41: _rust_maybe_catch_panic
  42: rustc_driver::pretty::print_after_hir_lowering
  43: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  44: std::sys::windows::thread::Thread::new
  45: BaseThreadInitThunk
  46: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (25d8a9494 2019-11-29) running on x86_64-pc-windows-msvc

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
