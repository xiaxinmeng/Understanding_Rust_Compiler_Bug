
$ build/x86_64-unknown-linux-gnu/llvm/bin/llc -march=aarch64 -global-isel=1 -O0 -mattr=-fp-armv8 test.ll                      
llc: /home/amanieu/code/rust/src/llvm-project/llvm/lib/CodeGen/GlobalISel/MachineIRBuilder.cpp:950: void llvm::MachineIRBuilder::validateTruncExt(llvm::LLT, llvm::LLT, bool): Assertion `DstTy.getSizeInBits() > SrcTy.getSizeInBits() && "invalid narrowing extend"' failed.
PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace.
Stack dump:
0.	Program arguments: build/x86_64-unknown-linux-gnu/llvm/bin/llc -march=aarch64 -global-isel=1 -O0 -mattr=-fp-armv8 test.ll
1.	Running pass 'Function Pass Manager' on module 'test.ll'.
2.	Running pass 'IRTranslator' on function '@foo'
 #0 0x0000563892935add llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x28f4add)
 #1 0x00005638929336f4 llvm::sys::RunSignalHandlers() (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x28f26f4)
 #2 0x0000563892933863 SignalHandler(int) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x28f2863)
 #3 0x00007fb774fbe960 __restore_rt (/usr/lib/libpthread.so.0+0x13960)
 #4 0x00007fb774ab0ef5 raise (/usr/lib/libc.so.6+0x3cef5)
 #5 0x00007fb774a9a862 abort (/usr/lib/libc.so.6+0x26862)
 #6 0x00007fb774a9a747 _nl_load_domain.cold (/usr/lib/libc.so.6+0x26747)
 #7 0x00007fb774aa9646 (/usr/lib/libc.so.6+0x35646)
 #8 0x0000563892eecd20 llvm::MachineIRBuilder::validateSelectOp(llvm::LLT, llvm::LLT, llvm::LLT, llvm::LLT) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x2eabd20)
 #9 0x0000563892eed0b3 llvm::MachineIRBuilder::buildInstr(unsigned int, llvm::ArrayRef<llvm::DstOp>, llvm::ArrayRef<llvm::SrcOp>, llvm::Optional<unsigned int>) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x2eac0b3)
#10 0x0000563890afca5f llvm::AArch64CallLowering::lowerReturn(llvm::MachineIRBuilder&, llvm::Value const*, llvm::ArrayRef<llvm::Register>, llvm::FunctionLoweringInfo&, llvm::Register) const (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0xabba5f)
#11 0x0000563892e82a57 llvm::IRTranslator::translateRet(llvm::User const&, llvm::MachineIRBuilder&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x2e41a57)
#12 0x0000563892e90044 llvm::IRTranslator::runOnMachineFunction(llvm::MachineFunction&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x2e4f044)
#13 0x0000563891c75a0c llvm::MachineFunctionPass::runOnFunction(llvm::Function&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x1c34a0c)
#14 0x000056389210b6d0 llvm::FPPassManager::runOnFunction(llvm::Function&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x20ca6d0)
#15 0x000056389210c211 llvm::FPPassManager::runOnModule(llvm::Module&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x20cb211)
#16 0x000056389210b033 llvm::legacy::PassManagerImpl::run(llvm::Module&) (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x20ca033)
#17 0x00005638907aaed7 main (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x769ed7)
#18 0x00007fb774a9bb25 __libc_start_main (/usr/lib/libc.so.6+0x27b25)
#19 0x0000563890847cae _start (build/x86_64-unknown-linux-gnu/llvm/bin/llc+0x806cae)
