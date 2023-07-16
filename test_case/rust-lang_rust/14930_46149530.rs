
* thread #2: tid = 0x48de0a, 0x00007fff83546866 libsystem_kernel.dylib`__pthread_kill + 10, stop reason = signal SIGABRT
  * frame #0: 0x00007fff83546866 libsystem_kernel.dylib`__pthread_kill + 10
    frame #1: 0x00007fff865e135c libsystem_pthread.dylib`pthread_kill + 92
    frame #2: 0x00000001016b3b16 librustc-d252d482-0.11.0-pre.dylib`abort + 22
    frame #3: 0x00000001016b3af1 librustc-d252d482-0.11.0-pre.dylib`__assert_rtn + 81
    frame #4: 0x000000010103c2fd librustc-d252d482-0.11.0-pre.dylib`llvm::DwarfDebug::constructAbstractSubprogramScopeDIE(llvm::DwarfCompileUnit&, llvm::LexicalScope*) + 349
    frame #5: 0x0000000101042ab9 librustc-d252d482-0.11.0-pre.dylib`llvm::DwarfDebug::endFunction(llvm::MachineFunction const*) + 1449
    frame #6: 0x00000001010234ad librustc-d252d482-0.11.0-pre.dylib`llvm::AsmPrinter::EmitFunctionBody() + 4637
    frame #7: 0x0000000100c3270a librustc-d252d482-0.11.0-pre.dylib`llvm::ARMAsmPrinter::runOnMachineFunction(llvm::MachineFunction&) + 362
    frame #8: 0x000000010112c41d librustc-d252d482-0.11.0-pre.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 125
    frame #9: 0x0000000101649bab librustc-d252d482-0.11.0-pre.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 347
    frame #10: 0x0000000101649e4b librustc-d252d482-0.11.0-pre.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 59
    frame #11: 0x000000010164a429 librustc-d252d482-0.11.0-pre.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1081
    frame #12: 0x000000010164a83d librustc-d252d482-0.11.0-pre.dylib`llvm::legacy::PassManager::run(llvm::Module&) + 13
    frame #13: 0x0000000100a71e23 librustc-d252d482-0.11.0-pre.dylib`LLVMRustWriteOutputFile + 307
    frame #14: 0x00000001008d71dd librustc-d252d482-0.11.0-pre.dylib`back::link::write_output_file::he9e93a5b7a268ff2wKf::v0.11.0.pre + 141
    frame #15: 0x00000001008dea30 librustc-d252d482-0.11.0-pre.dylib`back::link::write::run_passes::closure.91419 + 240
    frame #16: 0x00000001008d976c librustc-d252d482-0.11.0-pre.dylib`back::link::write::run_passes::hbb695e52f514c879MMf::v0.11.0.pre + 9356
    frame #17: 0x0000000100a20e01 librustc-d252d482-0.11.0-pre.dylib`driver::driver::phase_5_run_llvm_passes::closure.97852 + 97
    frame #18: 0x0000000100989c7a librustc-d252d482-0.11.0-pre.dylib`driver::driver::phase_5_run_llvm_passes::h7e7bf09cf59529cafxv::v0.11.0.pre + 490
    frame #19: 0x000000010097f380 librustc-d252d482-0.11.0-pre.dylib`driver::driver::compile_input::hf1998b432ab27ab2gbv::v0.11.0.pre + 7456
    frame #20: 0x0000000100a406ee librustc-d252d482-0.11.0-pre.dylib`driver::run_compiler::hb2048e6d930cfe5fOSx::v0.11.0.pre + 9662
    frame #21: 0x0000000100a3e076 librustc-d252d482-0.11.0-pre.dylib`driver::main_args::closure.98543 + 70
    frame #22: 0x0000000100a58397 librustc-d252d482-0.11.0-pre.dylib`driver::monitor::closure.99633 + 199
    frame #23: 0x0000000100a535bb librustc-d252d482-0.11.0-pre.dylib`task::TaskBuilder::try::closure.99396 + 75
    frame #24: 0x00000001000459ac libnative-1fb5e2c0-0.11.0-pre.dylib`task::spawn_opts::closure.7247 + 76
    frame #25: 0x000000010327b928 librustrt-d8560cb2-0.11.0-pre.dylib`task::Task::run::closure.5244 + 56
    frame #26: 0x00000001032f1c2c librustrt-d8560cb2-0.11.0-pre.dylib`rust_try + 12
    frame #27: 0x000000010327e23a librustrt-d8560cb2-0.11.0-pre.dylib`unwind::try::habea3eb6fbe31bfdLCd::v0.11.0.pre + 74
    frame #28: 0x000000010327b7f5 librustrt-d8560cb2-0.11.0-pre.dylib`task::Task::run::h31070b5f512029c7XSc::v0.11.0.pre + 101
    frame #29: 0x000000010004581b libnative-1fb5e2c0-0.11.0-pre.dylib`task::spawn_opts::closure.7219 + 267
    frame #30: 0x000000010327d879 librustrt-d8560cb2-0.11.0-pre.dylib`thread::thread_start::hfa0dfbb777e33e5dmad::v0.11.0.pre + 41
    frame #31: 0x00007fff865e0899 libsystem_pthread.dylib`_pthread_body + 138
    frame #32: 0x00007fff865e072a libsystem_pthread.dylib`_pthread_start + 137
