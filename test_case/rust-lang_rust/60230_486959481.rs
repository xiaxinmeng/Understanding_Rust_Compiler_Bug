
error: internal compiler error: src\librustc_mir\monomorphize\collector.rs:745: Cannot create local mono-item for DefId(11/0:14 ~ lazy_static[6522]::lazy[0]::{{impl}}[0]::get[0])

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:620:9
    Checking matrixmultiply v0.2.2
warning: not embedding natvis: lld-link may not support the flag

stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate<'tcx>> as rustc::ty::ToPredicate<'tcx>>::to_predicate
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::emitter::ColorConfig as core::fmt::Debug>::fmt
   6: rustc_errors::Handler::bug
   7: rustc::util::bug::bug_fmt
   8: rustc::ty::wf::object_region_bounds
   9: rustc::ty::wf::object_region_bounds
  10: rustc::ty::wf::object_region_bounds
  11: rustc::util::bug::bug_fmt
  12: rustc::util::bug::bug_fmt
  13: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_static
  14: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_static
  15: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  16: rustc_mir::monomorphize::collector::collect_crate_mono_items
  17: rustc_mir::monomorphize::collector::collect_crate_mono_items
  18: rustc_mir::monomorphize::collector::collect_crate_mono_items
  19: rustc_mir::monomorphize::collector::collect_crate_mono_items
  20: rustc_mir::monomorphize::collector::collect_crate_mono_items
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items
  22: rustc_mir::monomorphize::collector::collect_crate_mono_items
  23: rustc_mir::monomorphize::collector::collect_crate_mono_items
  24: rustc_mir::monomorphize::collector::collect_crate_mono_items
  25: <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass
  26: <rustc_mir::interpret::eval_context::StackPopCleanup as core::fmt::Debug>::fmt
  27: rustc_mir::monomorphize::collector::collect_crate_mono_items
  28: <rustc_mir::interpret::eval_context::StackPopCleanup as core::fmt::Debug>::fmt
  29: rustc_mir::monomorphize::partitioning::compute_codegen_unit_name
  30: <rustc_codegen_ssa::common::TypeKind as core::fmt::Debug>::fmt
  31: <rustc_codegen_ssa::MemFlags as core::fmt::UpperHex>::fmt
  32: <rustc_codegen_ssa::MemFlags as core::fmt::UpperHex>::fmt
  33: <rustc_codegen_ssa::MemFlags as core::fmt::UpperHex>::fmt
  34: rustc_codegen_ssa::back::archive::find_library
  35: <rustc_codegen_llvm::back::lto::ThinLTOImports as core::fmt::Debug>::fmt
  36: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  37: <rustc_codegen_llvm::llvm_::ffi::PassKind as core::fmt::Debug>::fmt
  38: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  39: <rustc_codegen_llvm::back::link::exec_linker::Escape<'a> as core::fmt::Display>::fmt
  40: rustc_codegen_llvm::consts::<impl rustc_codegen_ssa::traits::statics::StaticMethods for rustc_codegen_llvm::context::CodegenCx<'ll, 'tcx>>::codegen_static
  41: rustc_codegen_llvm::llvm_::diagnostic::Diagnostic::unpack
  42: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  43: rand::os::OsRng::new
  44: rustc_driver::driver::phase_4_codegen
  45: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  46: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  47: <env_logger::fmt::WriteStyle as core::default::Default>::default
  48: rustc_driver::driver::compile_input
  49: rustc_driver::run_compiler
  50: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  51: rustc_driver::run_compiler
  52: <unknown>
  53: <unknown>
  54: _rust_maybe_catch_panic
  55: <unknown>
  56: std::sys::windows::thread::Thread::new
  57: BaseThreadInitThunk
  58: RtlUserThreadStart
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [backend_optimization_level] optimization level used by backend
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.1 (fc50f328b 2019-04-24) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C linker=lld-link.exe -C link-args=-fuse-ld=lld -C opt-level=z -C inline-threshold=275 -C lto=no --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `thread_local`.
