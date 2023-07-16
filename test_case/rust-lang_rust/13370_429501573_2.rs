
   Compiling playground v0.0.1 (/playground)
thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:355:21
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
             at libstd/panicking.rs:480
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::panicking::panic
             at libcore/panicking.rs:52
  10: rustc_codegen_llvm::llvm::diagnostic::Diagnostic::unpack
  11: rustc_codegen_llvm::back::write::diagnostic_handler
  12: _ZN4llvm17DiagnosticHandler17handleDiagnosticsERKNS_14DiagnosticInfoE
  13: _ZN4llvm11LLVMContext8diagnoseERKNS_14DiagnosticInfoE
  14: _ZNK4llvm10AsmPrinter13EmitInlineAsmEPKNS_12MachineInstrE
  15: _ZN4llvm10AsmPrinter16EmitFunctionBodyEv
  16: _ZN4llvm13X86AsmPrinter20runOnMachineFunctionERNS_15MachineFunctionE
  17: _ZN4llvm19MachineFunctionPass13runOnFunctionERNS_8FunctionE
  18: _ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE
  19: _ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE
  20: _ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE
  21: LLVMRustWriteOutputFile
  22: rustc_codegen_llvm::back::write::write_output_file
  23: rustc_codegen_llvm::back::write::codegen::{{closure}}
  24: rustc::util::common::time_ext
  25: rustc_codegen_llvm::back::write::codegen
  26: rustc_codegen_llvm::back::write::execute_work_item
query stack during panic:
end of query stack
error: aborting due to worker thread failure

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
