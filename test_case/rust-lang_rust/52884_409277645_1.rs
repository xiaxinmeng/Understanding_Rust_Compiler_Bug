
#0  0x00007ffff713ffcf in raise () from /lib/x86_64-linux-gnu/libc.so.6
#1  0x00007ffff71413fa in abort () from /lib/x86_64-linux-gnu/libc.so.6
#2  0x00007ffff7138e37 in __assert_fail_base () from /lib/x86_64-linux-gnu/libc.so.6
#3  0x00007ffff7138ee2 in __assert_fail () from /lib/x86_64-linux-gnu/libc.so.6
#4  0x0000000002ba17b6 in llvm::IRTranslator::allocateVRegs(llvm::Value const&) ()
#5  0x0000000002ba073c in llvm::IRTranslator::translateExtractValue(llvm::User const&, llvm::MachineIRBuilder&) ()
#6  0x0000000002b97a34 in llvm::IRTranslator::translate(llvm::Instruction const&) ()
#7  0x0000000002b95c08 in llvm::IRTranslator::runOnMachineFunction(llvm::MachineFunction&) ()
#8  0x0000000001f24666 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
#9  0x00000000022fa482 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
#10 0x00000000022f9d78 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
#11 0x00000000022fbe50 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
#12 0x0000000001572fc7 in ?? ()
#13 0x000000000156f2ab in main ()
