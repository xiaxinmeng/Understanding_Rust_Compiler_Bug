gdb
(lldb) bt
* thread #5, name = 'rustc', stop reason = hit program assert
    frame #0: 0x00007fffee87dfff libc.so.6`__GI_raise(sig=6) at raise.c:51
    frame #1: 0x00007fffee87f42a libc.so.6`__GI_abort at abort.c:89
    frame #2: 0x00007fffee876e67 libc.so.6`__assert_fail_base(fmt="", assertion=<unavailable>, file=<unavailable>, line=<unavailable>, function=<unavailable>) at assert.c:92
    frame #3: 0x00007fffee876f12 libc.so.6`__GI___assert_fail(assertion=<unavailable>, file=<unavailable>, line=<unavailable>, function=<unavailable>) at assert.c:101
  * frame #4: 0x00007ffff04c26a3 librustc_driver-ff4867624e9288b7.so`llvm::TargetLoweringBase::getRegClassFor(isDivergent=<unavailable>, VT=<unavailable>, this=<unavailable>) const at TargetLowering.h:831:5
    frame #5: 0x00007ffff04ce8b2 librustc_driver-ff4867624e9288b7.so`llvm::MipsTargetLowering::parseRegForInlineAsmConstraint(llvm::StringRef, llvm::MVT) const at MipsISelLowering.cpp:4099:3
    frame #6: 0x00007ffff04ce8ad librustc_driver-ff4867624e9288b7.so`llvm::MipsTargetLowering::parseRegForInlineAsmConstraint(this=<unavailable>, C=<unavailable>, VT=<unavailable>) const at MipsISelLowering.cpp:4084
    frame #7: 0x00007ffff04e3da0 librustc_driver-ff4867624e9288b7.so`llvm::MipsTargetLowering::getRegForInlineAsmConstraint(this=0x00007fffd80229b0, TRI=0x00007fffd80217c0, Constraint=(Data = "{$8}", Length = 4), VT=(SimpleTy = i8)) const at MipsISelLowering.cpp:4165:54
    frame #8: 0x00007ffff12b0242 librustc_driver-ff4867624e9288b7.so`::GetRegistersForValue(DAG=0x00007fffd8001cb0, DL=0x00007fffecb17260, OpInfo=0x00007fffecb173c0, RefOpInfo=0x00007fffecb173c0)::SDISelAsmOperandInfo &, (anonymous namespace)::SDISelAsmOperandInfo &) at SelectionDAGBuilder.cpp:7954:61
    frame #9: 0x00007ffff12eafe2 librustc_driver-ff4867624e9288b7.so`llvm::SelectionDAGBuilder::visitInlineAsm(this=0x00007fffd8002780, Call=0x00007fffe81c7060) at SelectionDAGBuilder.cpp:8260:25
    frame #10: 0x00007ffff12fc223 librustc_driver-ff4867624e9288b7.so`llvm::SelectionDAGBuilder::visit(this=0x00007fffd8002780, I=0x00007fffe81c7060) at SelectionDAGBuilder.cpp:1141:8
    frame #11: 0x00007ffff135e619 librustc_driver-ff4867624e9288b7.so`llvm::SelectionDAGISel::SelectBasicBlock(this=0x00007fffd80016c0, Begin=<unavailable>, End=llvm::BasicBlock::const_iterator @ rbx, HadTailCall=0x00007fffecb18db0) at SelectionDAGISel.cpp:699:17
    frame #12: 0x00007ffff1362c62 librustc_driver-ff4867624e9288b7.so`llvm::SelectionDAGISel::SelectAllBasicBlocks(this=0x00007fffd80016c0, Fn=0x00007fffe81baa28) at SelectionDAGISel.cpp:1520:27
    frame #13: 0x00007ffff13640d0 librustc_driver-ff4867624e9288b7.so`llvm::SelectionDAGISel::runOnMachineFunction(this=0x00007fffd80016c0, mf=0x00007fffd805c030) at SelectionDAGISel.cpp:504:23
    frame #14: 0x00007ffff04c20fb librustc_driver-ff4867624e9288b7.so`llvm::MipsDAGToDAGISel::runOnMachineFunction(this=0x00007fffd80016c0, MF=0x00007fffd805c030) at MipsISelDAGToDAG.cpp:58:52
    frame #15: 0x00007ffff16735a5 librustc_driver-ff4867624e9288b7.so`llvm::MachineFunctionPass::runOnFunction(this=0x00007fffd80016c0, F=0x00007fffe81baa28) at MachineFunctionPass.cpp:73:33
    frame #16: 0x00007ffff23f1b33 librustc_driver-ff4867624e9288b7.so`llvm::FPPassManager::runOnFunction(this=0x00007fffd8007fb0, F=0x00007fffe81baa28) at LegacyPassManager.cpp:1587:40
    frame #17: 0x00007ffff23f2181 librustc_driver-ff4867624e9288b7.so`llvm::FPPassManager::runOnModule(this=0x00007fffd8007fb0, M=0x00007fffe8101080) at LegacyPassManager.cpp:1629:29
    frame #18: 0x00007ffff23f0ace librustc_driver-ff4867624e9288b7.so`llvm::legacy::PassManagerImpl::run(llvm::Module&) at LegacyPassManager.cpp:1698:38
    frame #19: 0x00007ffff23f0861 librustc_driver-ff4867624e9288b7.so`llvm::legacy::PassManagerImpl::run(this=0x00007fffd8000960, M=0x00007fffe8101080) at LegacyPassManager.cpp:614
    frame #20: 0x00007fffefd9cf55 librustc_driver-ff4867624e9288b7.so`LLVMRustWriteOutputFile + 581
    frame #21: 0x00007fffefd69548 librustc_driver-ff4867624e9288b7.so`rustc_codegen_llvm::back::write::write_output_file::h8f8e8512ef3d7b66 + 88
    frame #22: 0x00007fffefc16907 librustc_driver-ff4867624e9288b7.so`rustc_codegen_llvm::back::write::codegen::with_codegen::h2785a5280d050ea5 + 119
    frame #23: 0x00007fffefd6cc73 librustc_driver-ff4867624e9288b7.so`rustc_codegen_llvm::back::write::codegen::h1dcdb5946b72520a + 2899
    frame #24: 0x00007fffefd0beeb librustc_driver-ff4867624e9288b7.so`rustc_codegen_ssa::back::write::finish_intra_module_work::h63ef7bb6d09628bd + 219
    frame #25: 0x00007fffefd06fa9 librustc_driver-ff4867624e9288b7.so`rustc_codegen_ssa::back::write::execute_work_item::hfe14f77f2358639e + 2745
    frame #26: 0x00007fffefc17ca2 librustc_driver-ff4867624e9288b7.so`std::sys_common::backtrace::__rust_begin_short_backtrace::h7e5ec54f2af1c982 + 194
    frame #27: 0x00007fffefd87c5a librustc_driver-ff4867624e9288b7.so`std::panicking::try::h9dc227032d219e6e + 42
    frame #28: 0x00007fffefc0435d librustc_driver-ff4867624e9288b7.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h0fb2f699d71f4606 + 93
    frame #29: 0x00007fffeec56278 libstd-43e93f8b75aa198d.so`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h2e60e5e0bf1169eb + 24
    frame #30: 0x00007fffeec6bd1a libstd-43e93f8b75aa198d.so`std::sys::unix::thread::Thread::new::thread_start::h46685771fe1620e2 + 26
    frame #31: 0x00007fffee00f4a4 libpthread.so.0`start_thread(arg=0x00007fffecb1b700) at pthread_create.c:456
    frame #32: 0x00007fffee933d0f libc.so.6`__clone at clone.S:97
