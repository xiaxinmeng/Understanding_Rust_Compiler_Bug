
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `0`', src\librustc\mir\interpret\value.rs:305:17
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSliceMut as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: rust_begin_unwind
   8: std::panicking::begin_panic_fmt
   9: <rustc_mir::shim::CallKind as core::fmt::Debug>::fmt
  10: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  11: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  12: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  13: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  14: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  15: rustc_mir::const_eval::const_eval_raw_provider
  16: <rustc_mir::build::ScopeId as core::fmt::Debug>::fmt
  17: <rustc_mir::shim::CallKind as core::fmt::Debug>::fmt
  18: <rustc_mir::hair::pattern::PatRange as core::fmt::Debug>::fmt
  19: <rustc_mir::shim::CallKind as core::fmt::Debug>::fmt
  20: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  21: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  22: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  23: <rustc_mir::transform::remove_noop_landing_pads::RemoveNoopLandingPads as rustc_mir::transform::MirPass>::run_pass
  24: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  25: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  26: rustc_mir::transform::run_passes
  27: rustc_mir::transform::run_passes
  28: rustc_mir::transform::run_passes
  29: rustc::middle::weak_lang_items::<impl rustc::ty::context::TyCtxt>::is_weak_lang_item
  30: rustc::dep_graph::graph::DepGraph::assert_ignored
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack
  32: rustc::ty::<impl rustc::ty::context::TyCtxt>::instance_mir
  33: rustc_mir::monomorphize::collector::collect_crate_mono_items
  34: rustc_mir::monomorphize::collector::collect_crate_mono_items
  35: rustc_mir::monomorphize::collector::collect_crate_mono_items
  36: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_terminator
  37: <rustc_mir::interpret::memory::AllocCheck as core::fmt::Debug>::fmt
  38: rustc_mir::monomorphize::collector::collect_crate_mono_items
  39: <rustc_mir::interpret::memory::AllocCheck as core::fmt::Debug>::fmt
  40: rustc_mir::monomorphize::partitioning::compute_codegen_unit_name
  41: <rustc_codegen_llvm::llvm_::archive_ro::Iter as core::ops::drop::Drop>::drop
  42: <rustc_codegen_llvm::back::lto::ThinLTOImports as core::fmt::Debug>::fmt
  43: rustc_codegen_llvm::llvm_::diagnostic::Diagnostic::unpack
  44: <rustc_codegen_llvm::llvm_::ffi::PassKind as core::fmt::Debug>::fmt
  45: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  46: rustc_interface::passes::QueryContext::print_stats
  47: rustc_interface::passes::QueryContext::print_stats
  48: <env_logger::filter::inner::Filter as core::fmt::Debug>::fmt
  49: rustc_interface::queries::Queries::ongoing_codegen
  50: rustc_driver::pretty::print_after_hir_lowering
  51: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  52: <rustc::hir::Expr as rustc_typeck::check::coercion::AsCoercionSite>::as_coercion_site
  53: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  54: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  55: _rust_maybe_catch_panic
  56: rustc_driver::pretty::print_after_hir_lowering
  57: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  58: std::sys::windows::thread::Thread::new
  59: BaseThreadInitThunk
  60: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (c8ea4ace9 2019-12-14) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [const_eval_raw] const-evaluating `svm_class::m_csvm_linear`
#1 [optimized_mir] processing `svm_class::m_csvm_linear`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

error: could not compile `ffsvm`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name svm_class tests\svm_class.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C debuginfo=2 --test -C metadata=4ed02486dfc162c8 -C extra-filename=-4ed02486dfc162c8 --out-dir D:\Development\Source\ffsvm-rust\target\debug\deps -C incremental=D:\Development\Source\ffsvm-rust\target\debug\incremental -L dependency=D:\Development\Source\ffsvm-rust\target\debug\deps --extern ffsvm=D:\Development\Source\ffsvm-rust\target\debug\deps\libffsvm-526e738465434a2b.rlib --extern packed_simd=D:\Development\Source\ffsvm-rust\target\debug\deps\libpacked_simd-6600a16e46b39a24.rlib --extern pest=D:\Development\Source\ffsvm-rust\target\debug\deps\libpest-4cd557cf2fa26b5a.rlib --extern pest_derive=D:\Development\Source\ffsvm-rust\target\debug\deps\pest_derive-1fd69ab1b51bb5e5.dll --extern rand=D:\Development\Source\ffsvm-rust\target\debug\deps\librand-b0b96e83c985670f.rlib --extern simd_aligned=D:\Development\Source\ffsvm-rust\target\debug\deps\libsimd_aligned-56341438da1a9b6a.rlib` (exit code: 101)
