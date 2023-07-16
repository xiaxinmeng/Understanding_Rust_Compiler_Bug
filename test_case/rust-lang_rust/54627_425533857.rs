
* thread #1, queue = 'com.apple.main-thread', stop reason = signal SIGSTOP
  * frame #0: 0x00007fff6dd86a16 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff6df4f589 libsystem_pthread.dylib`_pthread_cond_wait + 732
    frame #2: 0x00000001055c1812 libstd-d913a14a4b766c72.dylib`std::thread::park::hcf011ba4ca76f30e + 258
    frame #3: 0x00000001055a5281 libstd-d913a14a4b766c72.dylib`std::sync::mpsc::blocking::WaitToken::wait::hd9230f94c5255b6a + 49
    frame #4: 0x00000001088b49e8 librustc_codegen_llvm-llvm.dylib`_$LT$std..sync..mpsc..shared..Packet$LT$T$GT$$GT$::recv::h283e3e15f08a3cef + 696
    frame #5: 0x0000000108997975 librustc_codegen_llvm-llvm.dylib`_$LT$std..sync..mpsc..Receiver$LT$T$GT$$GT$::recv::h4dd9e0364b3dd5e7 + 421
    frame #6: 0x00000001088d8c55 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::SharedEmitterMain::check::h54904d87a53fe4c4 (.llvm.15361068577713408237) + 133
    frame #7: 0x00000001088d90ff librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::OngoingCodegen::join::h309f3893243fe16d + 79
    frame #8: 0x0000000108992dd8 librustc_codegen_llvm-llvm.dylib`_$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_utils..codegen_backend..CodegenBackend$GT$::join_codegen_and_link::h4cd23f97545295e4 + 120
    frame #9: 0x0000000101f4ef78 librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::driver::compile_input::h31b03e47e946fdc9 + 9784
    frame #10: 0x0000000101fd86ff librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::run_compiler_with_pool::he921be7da2482371 + 4191
    frame #11: 0x0000000101f4c8dc librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::driver::spawn_thread_pool::h5e1b71cbb1fbfc82 + 396
    frame #12: 0x0000000101fd7592 librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::run_compiler::he50aeac7b58575a1 + 338
    frame #13: 0x0000000101f3baa6 librustc_driver-0d3b4d9a9612e751.dylib`_$LT$scoped_tls..ScopedKey$LT$T$GT$$GT$::set::h4a4b94afd31a07ef + 342
    frame #14: 0x0000000101f70e5e librustc_driver-0d3b4d9a9612e751.dylib`syntax::with_globals::h1e435231748ee4ed + 46
    frame #15: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #16: 0x0000000101fd58da librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::run::h580060b09b17efd1 + 2698
    frame #17: 0x0000000101fe3b3e librustc_driver-0d3b4d9a9612e751.dylib`rustc_driver::main::ha94c4dc722795849 + 14
    frame #18: 0x0000000101ecbdf6 rustc`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0ef2149b533e1701 + 6
    frame #19: 0x00000001055c9808 libstd-d913a14a4b766c72.dylib`std::panicking::try::do_call::h32a103bebbb9cfdd (.llvm.3970480468671491771) + 24
    frame #20: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #21: 0x00000001055b658d libstd-d913a14a4b766c72.dylib`std::rt::lang_start_internal::hb521492a178b86d1 + 237
    frame #22: 0x0000000101ecbe5c rustc`main + 44
    frame #23: 0x0000000101ecbde4 rustc`start + 52
  thread #2
    frame #0: 0x00007fff6dd86a16 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff6df4f589 libsystem_pthread.dylib`_pthread_cond_wait + 732
    frame #2: 0x00000001055c1812 libstd-d913a14a4b766c72.dylib`std::thread::park::hcf011ba4ca76f30e + 258
    frame #3: 0x00000001055a5281 libstd-d913a14a4b766c72.dylib`std::sync::mpsc::blocking::WaitToken::wait::hd9230f94c5255b6a + 49
    frame #4: 0x0000000103f00950 librustc-0d27f23848274e05.dylib`_$LT$std..sync..mpsc..stream..Packet$LT$T$GT$$GT$::recv::h201dca87373605ec + 528
    frame #5: 0x0000000103f07831 librustc-0d27f23848274e05.dylib`_$LT$std..sync..mpsc..IntoIter$LT$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$::next::hee0f59966d104b6f + 161
    frame #6: 0x0000000103f0275c librustc-0d27f23848274e05.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h4f0dc585bd4fead9 + 140
    frame #7: 0x0000000103f02bc0 librustc-0d27f23848274e05.dylib`std::panicking::try::do_call::h74f170cd9fa5f506 (.llvm.8886808114753586241) + 80
    frame #8: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #9: 0x0000000103f046f6 librustc-0d27f23848274e05.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hee0f7e7a3aae8011 + 230
    frame #10: 0x00000001055c8a48 libstd-d913a14a4b766c72.dylib`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #11: 0x000000010559e0a9 libstd-d913a14a4b766c72.dylib`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #12: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #13: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #14: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13
  thread #3
    frame #0: 0x00007fff6dd86a16 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff6df4f589 libsystem_pthread.dylib`_pthread_cond_wait + 732
    frame #2: 0x00000001055c1812 libstd-d913a14a4b766c72.dylib`std::thread::park::hcf011ba4ca76f30e + 258
    frame #3: 0x00000001055a5281 libstd-d913a14a4b766c72.dylib`std::sync::mpsc::blocking::WaitToken::wait::hd9230f94c5255b6a + 49
    frame #4: 0x00000001088b4e3d librustc_codegen_llvm-llvm.dylib`_$LT$std..sync..mpsc..shared..Packet$LT$T$GT$$GT$::recv::h96cd84ca2d712ca0 + 701
    frame #5: 0x00000001089972e7 librustc_codegen_llvm-llvm.dylib`_$LT$std..sync..mpsc..Receiver$LT$T$GT$$GT$::recv::h26367cdc3ee01033 + 407
    frame #6: 0x00000001088b0d35 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::he5435356e0826bb5 + 4053
    frame #7: 0x00000001088c6d0c librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::ha982ac7fd18c4a8f (.llvm.15361068577713408237) + 60
    frame #8: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #9: 0x00000001088ba957 librustc_codegen_llvm-llvm.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hf0d7cba651529e61 + 167
    frame #10: 0x00000001055c8a48 libstd-d913a14a4b766c72.dylib`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #11: 0x000000010559e0a9 libstd-d913a14a4b766c72.dylib`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #12: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #13: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #14: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13
  thread #4
    frame #0: 0x00000001098d70e8 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAG::getNode(unsigned int, llvm::SDLoc const&, llvm::EVT, llvm::SDValue, llvm::SDValue, llvm::SDNodeFlags) + 8
    frame #1: 0x000000010956c567 librustc_codegen_llvm-llvm.dylib`llvm::ARMTargetLowering::PerformCMOVCombine(llvm::SDNode*, llvm::SelectionDAG&) const + 4023
    frame #2: 0x000000010956dbd0 librustc_codegen_llvm-llvm.dylib`llvm::ARMTargetLowering::PerformDAGCombine(llvm::SDNode*, llvm::TargetLowering::DAGCombinerInfo&) const + 4224
    frame #3: 0x000000010975cee7 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::DAGCombiner::combine(llvm::SDNode*) + 151
    frame #4: 0x000000010975c3e4 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level) + 1764
    frame #5: 0x0000000109915898 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::CodeGenAndEmitDAG() + 1544
    frame #6: 0x00000001099140c2 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 8370
    frame #7: 0x000000010991158e librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1822
    frame #8: 0x000000010951c214 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::ARMDAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 20
    frame #9: 0x0000000109ad3e2a librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 138
    frame #10: 0x000000010a3961ee librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 974
    frame #11: 0x000000010a396453 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #12: 0x000000010a3967d7 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 695
    frame #13: 0x00000001089eeab6 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 550
    frame #14: 0x00000001088cce56 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h51aa72d49c2e043b (.llvm.15361068577713408237) + 86
    frame #15: 0x00000001088a7339 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h41cb86b680518476 (.llvm.13810846407243116275) + 761
    frame #16: 0x00000001088a0970 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::hcd8123cb24f97d34 + 80
    frame #17: 0x00000001088cf3f8 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::hf98753a3e7904c2f + 2376
    frame #18: 0x00000001088d6ab4 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::execute_work_item::h30fc8218f4df7441 + 6228
    frame #19: 0x00000001088afb6d librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h2cbf516cbc3e6fc0 + 541
    frame #20: 0x00000001088c6d5e librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hdccc49ad90c109bf (.llvm.15361068577713408237) + 46
    frame #21: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #22: 0x00000001088ba7a7 librustc_codegen_llvm-llvm.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::haa21b6c030791e16 + 167
    frame #23: 0x00000001055c8a48 libstd-d913a14a4b766c72.dylib`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #24: 0x000000010559e0a9 libstd-d913a14a4b766c72.dylib`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #25: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #26: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #27: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13
  thread #5
    frame #0: 0x000000010975b93d librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::DAGCombiner::AddToWorklist(llvm::SDNode*) + 125
    frame #1: 0x000000010975c4bc librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level) + 1980
    frame #2: 0x0000000109915898 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::CodeGenAndEmitDAG() + 1544
    frame #3: 0x00000001099140c2 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 8370
    frame #4: 0x000000010991158e librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1822
    frame #5: 0x000000010951c214 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::ARMDAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 20
    frame #6: 0x0000000109ad3e2a librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 138
    frame #7: 0x000000010a3961ee librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 974
    frame #8: 0x000000010a396453 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #9: 0x000000010a3967d7 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 695
    frame #10: 0x00000001089eeab6 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 550
    frame #11: 0x00000001088cce56 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h51aa72d49c2e043b (.llvm.15361068577713408237) + 86
    frame #12: 0x00000001088a7339 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h41cb86b680518476 (.llvm.13810846407243116275) + 761
    frame #13: 0x00000001088a0970 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::hcd8123cb24f97d34 + 80
    frame #14: 0x00000001088cf3f8 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::hf98753a3e7904c2f + 2376
    frame #15: 0x00000001088d6ab4 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::execute_work_item::h30fc8218f4df7441 + 6228
    frame #16: 0x00000001088afb6d librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h2cbf516cbc3e6fc0 + 541
    frame #17: 0x00000001088c6d5e librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hdccc49ad90c109bf (.llvm.15361068577713408237) + 46
    frame #18: 0x00000001055da62f libstd-d913a14a4b766c72.dylib`__rust_maybe_catch_panic + 31
    frame #19: 0x00000001088ba7a7 librustc_codegen_llvm-llvm.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::haa21b6c030791e16 + 167
    frame #20: 0x00000001055c8a48 libstd-d913a14a4b766c72.dylib`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #21: 0x000000010559e0a9 libstd-d913a14a4b766c72.dylib`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #22: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #23: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #24: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13
