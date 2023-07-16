
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 218', ../src/libcollections/vec.rs:1307
stack backtrace:
   1:     0x7ff481a51313 - std::sys::backtrace::tracing::imp::write::h4b09e6e8c01db097
   2:     0x7ff481a61f4d - std::panicking::default_hook::{{closure}}::h1d3243f546573ff4
   3:     0x7ff481a604ee - std::panicking::default_hook::h96c288d728df3ebf
   4:     0x7ff481a60bd8 - std::panicking::rust_panic_with_hook::hb1322e5f2588b4db
   5:     0x7ff481a60a72 - std::panicking::begin_panic::hfbeda5aad583dc32
   6:     0x7ff481a609b0 - std::panicking::begin_panic_fmt::h4fe9fb9d5109c4bf
   7:     0x7ff481a60931 - rust_begin_unwind
   8:     0x7ff481acb09f - core::panicking::panic_fmt::h4395919ece15c671
   9:     0x7ff481acb043 - core::panicking::panic_bounds_check::he139831fe8feb00a
  10:     0x7ff47a771222 - syntax::parse::token::InternedString::new_from_name::h4e84b67e8eab9873
  11:     0x7ff47a633efc - <syntax::ast::Name as core::fmt::Display>::fmt::h2e63739f57aaa2a4
  12:     0x7ff481ad6a45 - core::fmt::write::hd60e09e1e0026aa8
  13:     0x7ff481a6a1e0 - collections::fmt::format::h326044acb8129e34
  14:     0x7ff47a66ae1a - syntax::codemap::CodeMap::macro_backtrace::h7ddebad11eae3ade
  15:     0x7ff47a66b13e - <syntax::codemap::CodeMap as rustc_errors::CodeMapper>::macro_backtrace::h2c1478630d665a2b
  16:     0x7ff47a37cd97 - rustc_errors::emitter::EmitterWriter::fix_multispan_in_std_macros::h646f7d9bec7cf7cc
  17:     0x7ff47a37a4cc - <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit::h239b7341f013ef82
  18:     0x7ff47a389698 - rustc_errors::Handler::emit::hc4c60aa4cff99ef6
  19:     0x7ff4812a27d2 - rustc::session::Session::span_err::h2ccbbfd099a81eb3
  20:     0x7ff4812d49a0 - rustc_trans::back::write::report_inline_asm::hc0d9501a16d7aadf
  21:     0x7ff4812d4a93 - rustc_trans::back::write::inline_asm_handler::hd1c2ab2401f4a876
  22:     0x7ff47c9b9a16 - _ZN12_GLOBAL__N_19AsmParser11DiagHandlerERKN4llvm12SMDiagnosticEPv
  23:     0x7ff47cc1879d - _ZNK4llvm9SourceMgr12PrintMessageERNS_11raw_ostreamENS_5SMLocENS0_8DiagKindERKNS_5TwineENS_8ArrayRefINS_7SMRangeEEENS8_INS_7SMFixItEEEb
  24:     0x7ff47cc1895b - _ZNK4llvm9SourceMgr12PrintMessageENS_5SMLocENS0_8DiagKindERKNS_5TwineENS_8ArrayRefINS_7SMRangeEEENS6_INS_7SMFixItEEEb
  25:     0x7ff47c9af5ee - _ZN12_GLOBAL__N_19AsmParser5ErrorEN4llvm5SMLocERKNS1_5TwineENS1_8ArrayRefINS1_7SMRangeEEE
  26:     0x7ff47c9d7bfb - _ZN4llvm11MCAsmParser8TokErrorERKNS_5TwineENS_8ArrayRefINS_7SMRangeEEE
  27:     0x7ff47c9b4b96 - _ZN12_GLOBAL__N_19AsmParser16parsePrimaryExprERPKN4llvm6MCExprERNS1_5SMLocE
  28:     0x7ff47c9b5f3b - _ZN12_GLOBAL__N_19AsmParser15parseExpressionERPKN4llvm6MCExprERNS1_5SMLocE
  29:     0x7ff47c9b4cf1 - _ZN12_GLOBAL__N_19AsmParser16parsePrimaryExprERPKN4llvm6MCExprERNS1_5SMLocE
  30:     0x7ff47c9b5f3b - _ZN12_GLOBAL__N_19AsmParser15parseExpressionERPKN4llvm6MCExprERNS1_5SMLocE
  31:     0x7ff47bd338d8 - _ZN12_GLOBAL__N_112X86AsmParser15ParseMemOperandEjN4llvm5SMLocE
  32:     0x7ff47bd3470c - _ZN12_GLOBAL__N_112X86AsmParser15ParseATTOperandEv
  33:     0x7ff47bd40dd4 - _ZN12_GLOBAL__N_112X86AsmParser16ParseInstructionERN4llvm20ParseInstructionInfoENS1_9StringRefENS1_5SMLocERNS1_15SmallVectorImplISt10unique_ptrINS1_18MCParsedAsmOperandESt14default_deleteIS8_EEEE
  34:     0x7ff47b7c6f11 - _ZN4llvm17MCTargetAsmParser16ParseInstructionERNS_20ParseInstructionInfoENS_9StringRefENS_8AsmTokenERNS_15SmallVectorImplISt10unique_ptrINS_18MCParsedAsmOperandESt14default_deleteIS7_EEEE
  35:     0x7ff47c9c4180 - _ZN12_GLOBAL__N_19AsmParser14parseStatementERNS_18ParseStatementInfoEPN4llvm23MCAsmParserSemaCallbackE
  36:     0x7ff47c9c6cbe - _ZN12_GLOBAL__N_19AsmParser3RunEbb
  37:     0x7ff47c0dea24 - _ZNK4llvm10AsmPrinter13EmitInlineAsmENS_9StringRefERKNS_15MCSubtargetInfoERKNS_15MCTargetOptionsEPKNS_6MDNodeENS_9InlineAsm10AsmDialectE
  38:     0x7ff47c0df223 - _ZNK4llvm10AsmPrinter13EmitInlineAsmEPKNS_12MachineInstrE
  39:     0x7ff47c0dbd22 - _ZN4llvm10AsmPrinter16EmitFunctionBodyEv
  40:     0x7ff47bd4547d - _ZN4llvm13X86AsmPrinter20runOnMachineFunctionERNS_15MachineFunctionE
  41:     0x7ff47c29175a - _ZN4llvm19MachineFunctionPass13runOnFunctionERNS_8FunctionE
  42:     0x7ff47cb6e236 - _ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE
  43:     0x7ff47cb6e27a - _ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE
  44:     0x7ff47cb6fe4e - _ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE
  45:     0x7ff47b5a7775 - LLVMRustWriteOutputFile
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/rustllvm/PassWrapper.cpp:489
  46:     0x7ff4812d34eb - rustc_trans::back::write::write_output_file::hdb4b691f4323c380
  47:     0x7ff481385fd0 - rustc_trans::back::write::optimize_and_codegen::{{closure}}::h81be959d30d696f6
  48:     0x7ff4812db33f - rustc_trans::back::write::execute_work_item::hb51afc6495293b14
  49:     0x7ff4812d5d69 - rustc_trans::back::write::run_passes::h56d72398bc66cc68
  50:     0x7ff481e1f38f - rustc_driver::driver::phase_5_run_llvm_passes::h09bb1c97ec3e74c9
  51:     0x7ff481e07d18 - rustc_driver::driver::compile_input::hc0edbed7edb3eb18
  52:     0x7ff481e337b6 - rustc_driver::run_compiler::h22d678d32fb7c300
  53:     0x7ff481d71683 - std::panicking::try::do_call::h4d040997e2efdaf3
  54:     0x7ff481a69e66 - __rust_maybe_catch_panic
  55:     0x7ff481d8f109 - <F as alloc::boxed::FnBox<A>>::call_box::hba0b436c79e56b23
  56:     0x7ff481a5ea20 - std::sys::thread::Thread::new::thread_start::h022e3887023c6290
  57:     0x7ff479b1f483 - start_thread
  58:     0x7ff4816aa6dc - clone
  59:                0x0 - <unknown>
