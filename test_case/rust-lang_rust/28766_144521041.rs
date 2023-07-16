
(lldb) r
Process 3153 launched: '/Users/acrichton/.multirust/toolchains/nightly/bin/rustc' (x86_64)
Assertion failed: (!empty()), function back, file /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/llvm/include/llvm/ADT/SmallVector.h, line 157.
Process 3153 stopped
* thread #2: tid = 0xfe28, 0x00007fff866c8286 libsystem_kernel.dylib`__pthread_kill + 10, name = 'rustc', stop reason = signal SIGABRT
    frame #0: 0x00007fff866c8286 libsystem_kernel.dylib`__pthread_kill + 10
libsystem_kernel.dylib`__pthread_kill:
->  0x7fff866c8286 <+10>: jae    0x7fff866c8290            ; <+20>
    0x7fff866c8288 <+12>: movq   %rax, %rdi
    0x7fff866c828b <+15>: jmp    0x7fff866c3c53            ; cerror_nocancel
    0x7fff866c8290 <+20>: retq   
(lldb) bt
* thread #2: tid = 0xfe28, 0x00007fff866c8286 libsystem_kernel.dylib`__pthread_kill + 10, name = 'rustc', stop reason = signal SIGABRT
  * frame #0: 0x00007fff866c8286 libsystem_kernel.dylib`__pthread_kill + 10
    frame #1: 0x00007fff8c1cd9f9 libsystem_pthread.dylib`pthread_kill + 90
    frame #2: 0x000000010321ab36 librustc_llvm-10cbabc2.dylib`abort + 22
    frame #3: 0x000000010321ab11 librustc_llvm-10cbabc2.dylib`__assert_rtn + 81
    frame #4: 0x00000001029acc4b librustc_llvm-10cbabc2.dylib`llvm::returnTypeIsEligibleForTailCall(llvm::Function const*, llvm::Instruction const*, llvm::ReturnInst const*, llvm::TargetLoweringBase const&) + 1995
    frame #5: 0x00000001029ac44c librustc_llvm-10cbabc2.dylib`llvm::isInTailCallPosition(llvm::ImmutableCallSite, llvm::TargetMachine const&) + 412
    frame #6: 0x00000001028eee72 librustc_llvm-10cbabc2.dylib`llvm::SelectionDAGBuilder::LowerCallTo(llvm::ImmutableCallSite, llvm::SDValue, bool, llvm::MachineBasicBlock*) + 802
    frame #7: 0x00000001028e07db librustc_llvm-10cbabc2.dylib`llvm::SelectionDAGBuilder::visitCall(llvm::CallInst const&) + 1899
    frame #8: 0x00000001028d6c1a librustc_llvm-10cbabc2.dylib`llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) + 74
    frame #9: 0x000000010291de45 librustc_llvm-10cbabc2.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 8117
    frame #10: 0x000000010291af11 librustc_llvm-10cbabc2.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1489
    frame #11: 0x00000001026812e4 librustc_llvm-10cbabc2.dylib`(anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 20
    frame #12: 0x0000000102a795cd librustc_llvm-10cbabc2.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 125
    frame #13: 0x0000000103182f5c librustc_llvm-10cbabc2.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 332
    frame #14: 0x00000001031831bb librustc_llvm-10cbabc2.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 59
    frame #15: 0x0000000103183616 librustc_llvm-10cbabc2.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 774
    frame #16: 0x00000001031839dd librustc_llvm-10cbabc2.dylib`llvm::legacy::PassManager::run(llvm::Module&) + 13
    frame #17: 0x000000010202b7a7 librustc_llvm-10cbabc2.dylib`LLVMRustWriteOutputFile + 359
    frame #18: 0x00000001003edc98 librustc_trans-10cbabc2.dylib`back::write::write_output_file::h7b6a0917c12edcb5T9c + 104
    frame #19: 0x00000001003f040b librustc_trans-10cbabc2.dylib`back::write::optimize_and_codegen::closure.46290 + 1195
    frame #20: 0x00000001003f8881 librustc_trans-10cbabc2.dylib`back::write::execute_work_item::h029ce2382e4b96b4aZd + 5857
    frame #21: 0x00000001003f1c12 librustc_trans-10cbabc2.dylib`back::write::run_passes::h22631cc38b7ec107oKd + 5762
    frame #22: 0x0000000100054340 librustc_driver-10cbabc2.dylib`driver::phase_5_run_llvm_passes::ha14186a8d79e2f35YPa + 272
    frame #23: 0x00000001000070b4 librustc_driver-10cbabc2.dylib`driver::compile_input::hbc83f2980ca7f4710ba + 6532
    frame #24: 0x0000000100169931 librustc_driver-10cbabc2.dylib`run_compiler::h33fbe472d39f4682rqc + 4929
    frame #25: 0x000000010016718c librustc_driver-10cbabc2.dylib`boxed::F.FnBox$LT$A$GT$::call_box::h4139096335926771660 + 348
    frame #26: 0x0000000100166a93 librustc_driver-10cbabc2.dylib`sys_common::unwind::try::try_fn::h18182466805119396236 + 51
    frame #27: 0x0000000103faf299 libstd-10cbabc2.dylib`__rust_try + 9
    frame #28: 0x0000000103fa3d21 libstd-10cbabc2.dylib`sys_common::unwind::try::inner_try::hac438d316ecc2316T6r + 97
    frame #29: 0x0000000100166c43 librustc_driver-10cbabc2.dylib`boxed::F.FnBox$LT$A$GT$::call_box::h9346591452946938098 + 227
    frame #30: 0x0000000103fb782e libstd-10cbabc2.dylib`sys::thread::Thread::new::thread_start::h770a3938c0de9490puw + 142
    frame #31: 0x00007fff8c1cc05a libsystem_pthread.dylib`_pthread_body + 131
    frame #32: 0x00007fff8c1cbfd7 libsystem_pthread.dylib`_pthread_start + 176
    frame #33: 0x00007fff8c1c93ed libsystem_pthread.dylib`thread_start + 13
``
