
~/t/kernel ❯❯❯ lldb rustc
(lldb) target create "rustc"
Current executable set to 'rustc' (x86_64).
(lldb) run  --crate-name core --edition=2018 /Users/tnishinaga/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C metadata=0e695d8bdba0e0fe -C extra-filename=-0e695d8bdba0e0fe --out-dir /Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps --target /Users/tnishinaga/tmp/kernel/aarch64-unknown-none.json -Z force-unstable-if-unmarked -L dependency=/Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps -L dependency=/Users/tnishinaga/tmp/kernel/target/debug/deps --cap-lints allow
Process 14891 launched: '/Users/tnishinaga/.cargo/bin/rustc' (x86_64)
Process 14891 stopped
* thread #2, stop reason = exec
    frame #0: 0x000000010026d000 dyld`_dyld_start
dyld`_dyld_start:
->  0x10026d000 <+0>: popq   %rdi
    0x10026d001 <+1>: pushq  $0x0
    0x10026d003 <+3>: movq   %rsp, %rbp
    0x10026d006 <+6>: andq   $-0x10, %rsp
Target 0: (rustc) stopped.
(lldb) c
Process 14891 resuming
{"artifact":"/Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps/core-0e695d8bdba0e0fe.d","emit":"dep-info"}
{"artifact":"/Users/tnishinaga/tmp/kernel/target/aarch64-unknown-none/debug/deps/libcore-0e695d8bdba0e0fe.rmeta","emit":"metadata"}
Process 14891 stopped
* thread #8, stop reason = EXC_BAD_ACCESS (code=1, address=0xfffffffffffffffe)
    frame #0: 0x00000001017c8bf8 librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::findAction(std::__1::vector<std::__1::pair<unsigned short, llvm::LegalizeActions::LegalizeAction>, std::__1::allocator<std::__1::pair<unsigned short, llvm::LegalizeActions::LegalizeAction> > > const&, unsigned int) + 120
librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::findAction:
->  0x1017c8bf8 <+120>: movb   0x2(%r14,%rcx), %dil
    0x1017c8bfd <+125>: movzbl %dil, %ecx
    0x1017c8c01 <+129>: leaq   0x118(%rip), %rdx         ; <+416>
    0x1017c8c08 <+136>: movslq (%rdx,%rcx,4), %rcx
Target 0: (rustc) stopped.


(lldb) bt
* thread #8, stop reason = EXC_BAD_ACCESS (code=1, address=0xfffffffffffffffe)
  * frame #0: 0x00000001017c8bf8 librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::findAction(std::__1::vector<std::__1::pair<unsigned short, llvm::LegalizeActions::LegalizeAction>, std::__1::allocator<std::__1::pair<unsigned short, llvm::LegalizeActions::LegalizeAction> > > const&, unsigned int) + 120
    frame #1: 0x00000001017c79d3 librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::findScalarLegalAction(llvm::InstrAspect const&) const + 483
    frame #2: 0x00000001017c7f9f librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::getAction(llvm::LegalityQuery const&) const + 239
    frame #3: 0x00000001017c8344 librustc_driver-232461863c811c0c.dylib`llvm::LegalizerInfo::getAction(llvm::MachineInstr const&, llvm::MachineRegisterInfo const&) const + 852
    frame #4: 0x00000001017a2c24 librustc_driver-232461863c811c0c.dylib`llvm::LegalizerHelper::legalizeInstrStep(llvm::MachineInstr&) + 148
    frame #5: 0x000000010179ec97 librustc_driver-232461863c811c0c.dylib`llvm::Legalizer::legalizeMachineFunction(llvm::MachineFunction&, llvm::LegalizerInfo const&, llvm::ArrayRef<llvm::GISelChangeObserver*>, llvm::LostDebugLocObserver&, llvm::MachineIRBuilder&) + 1671
    frame #6: 0x000000010179fe30 librustc_driver-232461863c811c0c.dylib`llvm::Legalizer::runOnMachineFunction(llvm::MachineFunction&) + 816
    frame #7: 0x0000000101bc477d librustc_driver-232461863c811c0c.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 285
    frame #8: 0x0000000102556ef8 librustc_driver-232461863c811c0c.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1064
    frame #9: 0x000000010255c933 librustc_driver-232461863c811c0c.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #10: 0x0000000102557459 librustc_driver-232461863c811c0c.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 937
    frame #11: 0x0000000100912dc2 librustc_driver-232461863c811c0c.dylib`LLVMRustWriteOutputFile + 722
    frame #12: 0x00000001008397ec librustc_driver-232461863c811c0c.dylib`rustc_codegen_llvm::back::write::write_output_file::hc91f3ead66ce99ac + 364
    frame #13: 0x000000010083e794 librustc_driver-232461863c811c0c.dylib`rustc_codegen_llvm::back::write::codegen::h5fbc5674fe655c46 + 5172
    frame #14: 0x000000010085ba9f librustc_driver-232461863c811c0c.dylib`rustc_codegen_ssa::back::write::finish_intra_module_work::h7952582946c0a789 + 223
    frame #15: 0x0000000100855d73 librustc_driver-232461863c811c0c.dylib`rustc_codegen_ssa::back::write::execute_work_item::h643b74baadb55b95 + 3267
    frame #16: 0x000000010078429f librustc_driver-232461863c811c0c.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::hccecd38fee239049 + 159
    frame #17: 0x0000000100785b88 librustc_driver-232461863c811c0c.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h435cab2a972e42f7 + 168
    frame #18: 0x000000010807b75d libstd-cf45c391193686b0.dylib`std::sys::unix::thread::Thread::new::thread_start::h93dd3097fa4fa219 + 45
    frame #19: 0x00007fff72757109 libsystem_pthread.dylib`_pthread_start + 148
    frame #20: 0x00007fff72752b8b libsystem_pthread.dylib`thread_start + 15
