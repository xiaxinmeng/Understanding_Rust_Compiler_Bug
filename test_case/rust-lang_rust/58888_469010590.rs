
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate<'tcx>> as rustc::ty::ToPredicate<'tcx>>::to_predicate
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::emitter::ColorConfig as core::fmt::Debug>::fmt
   6: rustc_errors::Handler::bug
   7: rustc::util::bug::bug_fmt
   8: rustc::ty::context::<impl rustc::infer::canonical::Canonical<'gcx, rustc::ty::context::UserType<'gcx>>>::is_identity
   9: rustc::ty::context::<impl rustc::infer::canonical::Canonical<'gcx, rustc::ty::context::UserType<'gcx>>>::is_identity
  10: rustc::ty::context::<impl rustc::infer::canonical::Canonical<'gcx, rustc::ty::context::UserType<'gcx>>>::is_identity
  11: rustc::util::bug::bug_fmt
  12: rustc::util::bug::bug_fmt
  13: <rustc_codegen_llvm::back::wasm::WasmSections<'a> as core::iter::traits::iterator::Iterator>::next
  14: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoBuilderMethods<'tcx> for rustc_codegen_llvm::builder::Builder<'a, 'll, 'tcx>>::declare_local
  15: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  16: <rustc_codegen_llvm::llvm_::ffi::debuginfo::DISPFlags as core::fmt::Debug>::fmt
  17: <unknown>
  18: rustc_codegen_llvm::llvm_::diagnostic::Diagnostic::unpack
  19: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  20: <rustc_codegen_llvm::base::ValueIter<'ll> as core::iter::traits::iterator::Iterator>::next
  21: <rustc_codegen_llvm::llvm_::ffi::PassKind as core::fmt::Debug>::fmt
  22: <rustc_codegen_llvm::base::ValueIter<'ll> as core::iter::traits::iterator::Iterator>::next
  23: rustc_codegen_llvm::llvm_::diagnostic::Diagnostic::unpack
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  25: <env_logger::fmt::WriteStyle as core::default::Default>::default
  26: rustc_driver::driver::phase_4_codegen
  27: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  28: <env_logger::Logger as log::Log>::flush
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler
  31: <env_logger::Logger as log::Log>::flush
  32: rustc_driver::run_compiler
  33: <env_logger::Logger as log::Log>::flush
  34: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  35: _rust_maybe_catch_panic
  36: <env_logger::Logger as log::Log>::flush
  37: std::sys::windows::thread::Thread::new
  38: BaseThreadInitThunk
  39: RtlUserThreadStart
query stack during panic:
end of query stack
