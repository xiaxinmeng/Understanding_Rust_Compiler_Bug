
(lldb) c
Process 4803 resuming
Process 4803 stopped
* thread #2, stop reason = EXC_BAD_ACCESS (code=1, address=0x10)
    frame #0: 0x00000001016ce319 librustc_driver-6503a362531c867a.dylib`(anonymous namespace)::MachineCSE::runOnMachineFunction(llvm::MachineFunction&) + 4201
librustc_driver-6503a362531c867a.dylib`(anonymous namespace)::MachineCSE::runOnMachineFunction:
->  0x1016ce319 <+4201>: movq   0x10(%rax), %rax
    0x1016ce31d <+4205>: cmpw   $0x10, (%rax)
    0x1016ce321 <+4209>: jne    0x1016ce2d0               ; <+4128>
    0x1016ce323 <+4211>: movq   0x20(%r12), %rax
Target 0: (rustc) stopped.
(lldb) p $rax
(unsigned long) $0 = 0
(lldb) disassemble
librustc_driver-6503a362531c867a.dylib`(anonymous namespace)::MachineCSE::runOnMachineFunction:
...
    0x1016ce2ef <+4159>:  movq   -0x30(%rbp), %r12
    0x1016ce2f3 <+4163>:  movq   0x88(%r12), %rdi
    0x1016ce2fb <+4171>:  movl   %r15d, %esi
    0x1016ce2fe <+4174>:  callq  0x10173d700               ; llvm::MachineRegisterInfo::hasOneNonDBGUse(unsigned int) const
    0x1016ce303 <+4179>:  movb   %al, -0x48(%rbp)
    0x1016ce306 <+4182>:  movq   0x88(%r12), %rdi
    0x1016ce30e <+4190>:  movl   %r15d, %esi
    0x1016ce311 <+4193>:  callq  0x10173d5e0               ; llvm::MachineRegisterInfo::getVRegDef(unsigned int) const
    0x1016ce316 <+4198>:  movq   %rax, %r12
->  0x1016ce319 <+4201>:  movq   0x10(%rax), %rax
    0x1016ce31d <+4205>:  cmpw   $0x10, (%rax)
    0x1016ce321 <+4209>:  jne    0x1016ce2d0               ; <+4128>
    0x1016ce323 <+4211>:  movq   0x20(%r12), %rax
    0x1016ce328 <+4216>:  movl   0x24(%rax), %ecx
    0x1016ce32b <+4219>:  testl  %ecx, %ecx
    0x1016ce32d <+4221>:  jns    0x1016ce2d0               ; <+4128>
    0x1016ce32f <+4223>:  testl  $0xfff00, (%rax)          ; imm = 0xFFF00
    0x1016ce335 <+4229>:  jne    0x1016ce2d0               ; <+4128>
    0x1016ce337 <+4231>:  testl  $0xfff00, 0x20(%rax)      ; imm = 0xFFF00
    0x1016ce33e <+4238>:  jne    0x1016ce2d0               ; <+4128>
    0x1016ce340 <+4240>:  movq   -0x30(%rbp), %rax
    0x1016ce344 <+4244>:  movq   0x88(%rax), %rdi
    0x1016ce34b <+4251>:  movl   %ecx, %esi
    0x1016ce34d <+4253>:  movl   %ecx, -0x38(%rbp)
    0x1016ce350 <+4256>:  movl   %r15d, %edx
    0x1016ce353 <+4259>:  xorl   %ecx, %ecx
    0x1016ce355 <+4261>:  callq  0x10173bf00               ; llvm::MachineRegisterInfo::constrainRegAttrs(unsigned int, unsigned int, unsigned int)
    0x1016ce35a <+4266>:  testb  %al, %al
...
(lldb) bt
* thread #2, stop reason = EXC_BAD_ACCESS (code=1, address=0x10)
  * frame #0: 0x00000001016ce319 librustc_driver-6503a362531c867a.dylib`(anonymous namespace)::MachineCSE::runOnMachineFunction(llvm::MachineFunction&) + 4201
    frame #1: 0x00000001016dec0d librustc_driver-6503a362531c867a.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 285
    frame #2: 0x0000000101f70271 librustc_driver-6503a362531c867a.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1057
    frame #3: 0x0000000101f70573 librustc_driver-6503a362531c867a.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 131
    frame #4: 0x0000000101f70a49 librustc_driver-6503a362531c867a.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 969
    frame #5: 0x0000000100692c93 librustc_driver-6503a362531c867a.dylib`LLVMRustWriteOutputFile + 563
    frame #6: 0x0000000100617f06 librustc_driver-6503a362531c867a.dylib`rustc_codegen_llvm::back::write::write_output_file::hc095d205b92ff551 (.llvm.11416920907895951992) + 86
    frame #7: 0x0000000100673ff8 librustc_driver-6503a362531c867a.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h550e26036a74232a (.llvm.2814728182923408674) + 1560
    frame #8: 0x0000000100672463 librustc_driver-6503a362531c867a.dylib`rustc::util::common::time_ext::hc32b343c971dafa4 + 163
    frame #9: 0x000000010061e403 librustc_driver-6503a362531c867a.dylib`_$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_ssa..traits..write..WriteBackendMethods$GT$::codegen::h4171fac8b00b20c5 + 3635
    frame #10: 0x000000010052e773 librustc_driver-6503a362531c867a.dylib`rustc_codegen_ssa::back::write::execute_work_item::h8258a8c1c35d32be + 499
    frame #11: 0x0000000100665a7f librustc_driver-6503a362531c867a.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h938c64ba5fbd7ad1 + 239
    frame #12: 0x000000010058bbcb librustc_driver-6503a362531c867a.dylib`std::panicking::try::do_call::h76276dd7b0e89039 (.llvm.2242646780768277059) + 43
    frame #13: 0x000000010616721f libstd-bd6d3f2e95da2fcf.dylib`__rust_maybe_catch_panic + 31
    frame #14: 0x0000000100518ef6 librustc_driver-6503a362531c867a.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h6b9bb22ee9c4c669 + 134
    frame #15: 0x000000010613878e libstd-bd6d3f2e95da2fcf.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h1933a4bdd3bf4ecd + 62
    frame #16: 0x0000000106165f5e libstd-bd6d3f2e95da2fcf.dylib`std::sys::unix::thread::Thread::new::thread_start::h8ae6457b30b94596 + 142
    frame #17: 0x00007fff5e4d52eb libsystem_pthread.dylib`_pthread_body + 126
    frame #18: 0x00007fff5e4d8249 libsystem_pthread.dylib`_pthread_start + 66
    frame #19: 0x00007fff5e4d440d libsystem_pthread.dylib`thread_start + 13
