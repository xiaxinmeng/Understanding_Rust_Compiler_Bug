
stack backtrace:
   1:     0x7fb32df949df - std::sys::backtrace::tracing::imp::write::h22f199c1dbb72ba2
   2:     0x7fb32dfa3f0d - std::panicking::default_hook::{{closure}}::h9a389c462b6a22dd
   3:     0x7fb32dfa1386 - std::panicking::default_hook::h852b4223c1c00c59
   4:     0x7fb32dfa1a68 - std::panicking::rust_panic_with_hook::hcd9d05f53fa0dafc
   5:     0x7fb32dfa1902 - std::panicking::begin_panic::hf6c488cee66e7f17
   6:     0x7fb32dfa1840 - std::panicking::begin_panic_fmt::hb0a7126ee57cdd27
   7:     0x7fb32dfa17c1 - rust_begin_unwind
   8:     0x7fb32dfef6df - core::panicking::panic_fmt::h9af671b78898cdba
   9:     0x7fb32dfef683 - core::panicking::panic_bounds_check::h56f656aa4e352200
  10:     0x7fb32a68a802 - syntax::ast::Name::as_str::h38b25568cfeb8bbd
  11:     0x7fb32a68a8cc - <syntax::ast::Name as core::fmt::Display>::fmt::h82dc8d66519a98d5
  12:     0x7fb32dff1975 - core::fmt::write::he349f75ae1aca1a1
  13:     0x7fb32dfac730 - collections::fmt::format::h4276955d08ba3cf1
  14:     0x7fb32a698fea - syntax::codemap::CodeMap::macro_backtrace::h34dc4db4c69f95f8
  15:     0x7fb32a69930e - <syntax::codemap::CodeMap as rustc_errors::CodeMapper>::macro_backtrace::h66edae518ba75521
  16:     0x7fb32a3de6e7 - rustc_errors::emitter::EmitterWriter::fix_multispan_in_std_macros::h64647a5ae27a40eb
  17:     0x7fb32a3dbda6 - <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit::hf391e61324782492
  18:     0x7fb32a3e8e38 - rustc_errors::Handler::emit::hb6211616feb773be
  19:     0x7fb32cb04232 - rustc::session::Session::span_err::hc0a7da3b1ecabc87
  20:     0x7fb32cb32750 - rustc_trans::back::write::report_inline_asm::hcc7f5c5fe2807a91
  21:     0x7fb32cb32843 - rustc_trans::back::write::inline_asm_handler::h024a8afb079c4f0b
  22:     0x7fb3283c83b6 - _ZN12_GLOBAL__N_19AsmParser11DiagHandlerERKN4llvm12SMDiagnosticEPv
  23:     0x7fb32862713d - _ZNK4llvm9SourceMgr12PrintMessageERNS_11raw_ostreamENS_5SMLocENS0_8DiagKindERKNS_5TwineENS_8ArrayRefINS_7SMRangeEEENS8_INS_7SMFixItEEEb
  24:     0x7fb3286272fb - _ZNK4llvm9SourceMgr12PrintMessageENS_5SMLocENS0_8DiagKindERKNS_5TwineENS_8ArrayRefINS_7SMRangeEEENS6_INS_7SMFixItEEEb
  25:     0x7fb3283bdf8e - _ZN12_GLOBAL__N_19AsmParser5ErrorEN4llvm5SMLocERKNS1_5TwineENS1_8ArrayRefINS1_7SMRangeEEE
  26:     0x7fb32773c8b4 - _ZN12_GLOBAL__N_112X86AsmParser19ErrorMissingFeatureEN4llvm5SMLocEmb
  27:     0x7fb327747d5d - _ZN12_GLOBAL__N_112X86AsmParser28MatchAndEmitIntelInstructionEN4llvm5SMLocERjRNS1_15SmallVectorImplISt10unique_ptrINS1_18MCParsedAsmOperandESt14default_deleteIS6_EEEERNS1_10MCStreamerERmb
  28:     0x7fb3283d46fa - _ZN12_GLOBAL__N_19AsmParser14parseStatementERNS_18ParseStatementInfoEPN4llvm23MCAsmParserSemaCallbackE
  29:     0x7fb3283d565e - _ZN12_GLOBAL__N_19AsmParser3RunEbb
  30:     0x7fb327aed544 - _ZNK4llvm10AsmPrinter13EmitInlineAsmENS_9StringRefERKNS_15MCSubtargetInfoERKNS_15MCTargetOptionsEPKNS_6MDNodeENS_9InlineAsm10AsmDialectE
  31:     0x7fb327aedd43 - _ZNK4llvm10AsmPrinter13EmitInlineAsmEPKNS_12MachineInstrE
  32:     0x7fb327aea842 - _ZN4llvm10AsmPrinter16EmitFunctionBodyEv
  33:     0x7fb327753f9d - _ZN4llvm13X86AsmPrinter20runOnMachineFunctionERNS_15MachineFunctionE
  34:     0x7fb327ca027a - _ZN4llvm19MachineFunctionPass13runOnFunctionERNS_8FunctionE
  35:     0x7fb32857cbd6 - _ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE
  36:     0x7fb32857cc1a - _ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE
  37:     0x7fb32857e7ee - _ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE
  38:     0x7fb326fb6285 - LLVMRustWriteOutputFile
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/rustllvm/PassWrapper.cpp:489
  39:     0x7fb32cb314f3 - rustc_trans::back::write::write_output_file::h7e8cf29735c33835
  40:     0x7fb32cbd7c3e - rustc_trans::back::write::optimize_and_codegen::{{closure}}::h368ad232bd936447
  41:     0x7fb32cb390ca - rustc_trans::back::write::execute_work_item::h11850bd68a6e29be
  42:     0x7fb32cb33b19 - rustc_trans::back::write::run_passes::h17498b78d8227324
  43:     0x7fb32e32adbf - rustc_driver::driver::phase_5_run_llvm_passes::h11938d38ef5db27c
  44:     0x7fb32e314232 - rustc_driver::driver::compile_input::h5b63ccd49eeeb98b
  45:     0x7fb32e33d2ba - rustc_driver::run_compiler::h98c7274e7cb1d11d
  46:     0x7fb32e276f0b - std::panicking::try::do_call::h99ed0da044e497c3
  47:     0x7fb32dfabe06 - __rust_maybe_catch_panic
  48:     0x7fb32e295461 - <F as alloc::boxed::FnBox<A>>::call_box::hbdd5a14cd8e33b97
  49:     0x7fb32df9fde0 - std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2
  50:     0x7fb3262836f9 - start_thread
  51:     0x7fb32dc63b5c - clone
  52:                0x0 - <unknown>
