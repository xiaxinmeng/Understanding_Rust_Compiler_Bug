
  * frame #0: 0x00007ffff71514b7 libc.so.6`__GI_raise + 55
    frame #1: 0x00007ffff715288a libc.so.6`__GI_abort + 362
    frame #2: 0x00007ffff34ff65c librustc_llvm-4e7c5e5c.so`llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) + 156
    frame #3: 0x00007ffff284a62d librustc_llvm-4e7c5e5c.so`llvm::ARMTargetLowering::getEffectiveCallingConv(unsigned int, bool) const + 77
    frame #4: 0x00007ffff284a75b librustc_llvm-4e7c5e5c.so`llvm::ARMTargetLowering::CCAssignFnForNode(unsigned int, bool, bool) const + 11
    frame #5: 0x00007ffff284a8a4 librustc_llvm-4e7c5e5c.so`llvm::ARMTargetLowering::CanLowerReturn(unsigned int, llvm::MachineFunction&, bool, llvm::SmallVectorImpl<llvm::ISD::OutputArg> const&, llvm::LLVMContext&) const + 116
    frame #6: 0x00007ffff2b0953c librustc_llvm-4e7c5e5c.so`llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*) + 220
    frame #7: 0x00007ffff2c490a6 librustc_llvm-4e7c5e5c.so`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 534
    frame #8: 0x00007ffff283ee10 librustc_llvm-4e7c5e5c.so`(anonymous namespace)::ARMDAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 48
    frame #9: 0x00007ffff349477f librustc_llvm-4e7c5e5c.so`llvm::FPPassManager::runOnFunction(llvm::Function&) + 623
    frame #10: 0x00007ffff34947bb librustc_llvm-4e7c5e5c.so`llvm::FPPassManager::runOnModule(llvm::Module&) + 43
    frame #11: 0x00007ffff349437f librustc_llvm-4e7c5e5c.so`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 815
    frame #12: 0x00007ffff232662e librustc_llvm-4e7c5e5c.so`LLVMRustWriteOutputFile(Target=0x00007fffe80356a0, PMR=0x00007fffe801b770, M=0x00007fffe8002d30, path=0x00007fffee8242b0, FileType=CGFT_ObjectFile) + 462 at PassWrapper.cpp:210
    frame #13: 0x00007ffff635844b librustc_trans-4e7c5e5c.so`back::write::write_output_file::h67e97df156a1fd13Rac + 187
    frame #14: 0x00007ffff635aea3 librustc_trans-4e7c5e5c.so`back::write::optimize_and_codegen::closure.40104 + 1315
    frame #15: 0x00007ffff6364dd4 librustc_trans-4e7c5e5c.so`back::write::execute_work_item::h565fc52094442fc5Y4c + 6372
    frame #16: 0x00007ffff635cb1d librustc_trans-4e7c5e5c.so`back::write::run_passes::h70fcfd6f6115b442KKc + 6925
    frame #17: 0x00007ffff7b003b8 librustc_driver-4e7c5e5c.so`driver::phase_5_run_llvm_passes::ha70ba0f00d6855749Oa + 328
    frame #18: 0x00007ffff7ad8849 librustc_driver-4e7c5e5c.so`driver::compile_input::h19dbb81f9d079a10Qba + 7849
    frame #19: 0x00007ffff7b90dc2 librustc_driver-4e7c5e5c.so`run_compiler::h6fdcc5ae4819da2c65b + 3826
    frame #20: 0x00007ffff7b8e613 librustc_driver-4e7c5e5c.so`boxed::F.FnBox$LT$A$GT$::call_box::h15867338134671801927 + 451
    frame #21: 0x00007ffff7b8dbda librustc_driver-4e7c5e5c.so`rt::unwind::try::try_fn::h10571205104630549427 + 74
    frame #22: 0x00007ffff76252f9 libstd-4e7c5e5c.so`rust_try_inner + 9
    frame #23: 0x00007ffff76252e6 libstd-4e7c5e5c.so`rust_try + 6
    frame #24: 0x00007ffff7b8de75 librustc_driver-4e7c5e5c.so`boxed::F.FnBox$LT$A$GT$::call_box::h11752498478748955554 + 405
    frame #25: 0x00007ffff75b07c2 libstd-4e7c5e5c.so`sys::thread::Thread::new::thread_start::h9f92b31902592327dAv + 146
    frame #26: 0x00007ffff1663374 libpthread.so.0`start_thread + 196
    frame #27: 0x00007ffff720627d libc.so.6`__clone + 109
