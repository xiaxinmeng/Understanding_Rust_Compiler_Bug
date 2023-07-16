
 	rustc_codegen_llvm-llvm.dll!common_assert_to_message_box<wchar_t>(const wchar_t * const expression, const wchar_t * const file_name, const unsigned int line_number, void * const return_address) Line 388	C++
>	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::lowerTypeMemberFunction(const llvm::DISubroutineType * Ty, const llvm::DIType * ClassTy, int ThisAdjustment, bool IsStaticMethod, llvm::codeview::FunctionOptions FO) Line 1839	C++
 	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::getMemberFunctionType(const llvm::DISubprogram * SP, const llvm::DICompositeType * Class) Line 420	C++
 	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::getFuncIdForSubprogram(const llvm::DISubprogram * SP) Line 354	C++
 	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::getInlineSite(const llvm::DILocation * InlinedAt, const llvm::DISubprogram * Inlinee) Line 244	C++
 	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::maybeRecordLocation(const llvm::DebugLoc & DL, const llvm::MachineFunction * MF) Line 491	C++
 	rustc_codegen_llvm-llvm.dll!llvm::CodeViewDebug::beginInstruction(const llvm::MachineInstr * MI) Line 2835	C++
 	rustc_codegen_llvm-llvm.dll!llvm::AsmPrinter::EmitFunctionBody() Line 1097	C++
 	rustc_codegen_llvm-llvm.dll!llvm::X86AsmPrinter::runOnMachineFunction(llvm::MachineFunction & MF) Line 81	C++
 	rustc_codegen_llvm-llvm.dll!llvm::MachineFunctionPass::runOnFunction(llvm::Function & F) Line 76	C++
 	rustc_codegen_llvm-llvm.dll!llvm::FPPassManager::runOnFunction(llvm::Function & F) Line 1644	C++
 	rustc_codegen_llvm-llvm.dll!llvm::FPPassManager::runOnModule(llvm::Module & M) Line 1679	C++
 	rustc_codegen_llvm-llvm.dll!`anonymous namespace'::MPPassManager::runOnModule(llvm::Module & M) Line 1744	C++
 	rustc_codegen_llvm-llvm.dll!llvm::legacy::PassManagerImpl::run(llvm::Module & M) Line 1857	C++
 	rustc_codegen_llvm-llvm.dll!LLVMRustWriteOutputFile()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN18rustc_codegen_llvm4back5write8llvm_err17h31cd68abf6d953a0E()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN18rustc_codegen_llvm4back5write7codegen17hfd1699f5ede2ea9eE()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN18rustc_codegen_llvm4back5write7codegen17hfd1699f5ede2ea9eE()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN17rustc_codegen_ssa4back5write17execute_work_item17h08f1f546452f8fa8E()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h8c3275823cf7bf83E()	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN3std9panicking3try7do_call17he2ac0cf2f19fa3e5E.llvm.4772412519095044490()	Unknown
 	std-fbf0d0b0a29eef3c.dll!panic_unwind::__rust_maybe_catch_panic(void (unsigned char *) * f, unsigned char * data, unsigned __int64 * data_ptr, unsigned __int64 * vtable_ptr) Line 92	Unknown
 	rustc_codegen_llvm-llvm.dll!_ZN4core6result13unwrap_failed17hf5b878ec117809f8E()	Unknown
 	std-fbf0d0b0a29eef3c.dll!std::sys::windows::thread::{{impl}}::new::thread_start(core::ffi::c_void * main) Line 47	Unknown
 	kernel32.dll!00007ffa0f213dc4()	Unknown
 	ntdll.dll!00007ffa113c3691()	Unknown
