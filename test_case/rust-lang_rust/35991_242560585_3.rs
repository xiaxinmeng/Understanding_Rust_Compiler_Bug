
#1  0x00007f3e0a02664a in __GI_abort () at abort.c:89
#2  0x00007f3e0a01e107 in __assert_fail_base (fmt=<optimized out>, 
    assertion=assertion@entry=0x7f3e060f10b8 "!isa<DIType>(Scope) && \"shouldn't make a namespace scope for a type\"", 
    file=file@entry=0x7f3e060f0e88 "/opt/rust/src/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp", 
    line=line@entry=202, 
    function=function@entry=0x7f3e060f8cc0 <llvm::CodeViewDebug::getScopeIndex(llvm::DIScope const*)::__PRETTY_FUNCTION__> "llvm::codeview::TypeIndex llvm::CodeViewDebug::getScopeIndex(const llvm::DIScope*)") at assert.c:92
#3  0x00007f3e0a01e1b2 in __GI___assert_fail (
    assertion=0x7f3e060f10b8 "!isa<DIType>(Scope) && \"shouldn't make a namespace scope for a type\"", 
    file=0x7f3e060f0e88 "/opt/rust/src/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp", line=202, 
    function=0x7f3e060f8cc0 <llvm::CodeViewDebug::getScopeIndex(llvm::DIScope const*)::__PRETTY_FUNCTION__> "llvm::codeview::TypeIndex llvm::CodeViewDebug::getScopeIndex(const llvm::DIScope*)") at assert.c:101
#4  0x00007f3e04e61054 in llvm::CodeViewDebug::getScopeIndex(llvm::DIScope const*) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#5  0x00007f3e04e66650 in llvm::CodeViewDebug::getFuncIdForSubprogram(llvm::DISubprogram const*) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#6  0x00007f3e04e6a7cc in llvm::CodeViewDebug::getInlineSite(llvm::DILocation const*, llvm::DISubprogram const*) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#7  0x00007f3e04e6ab52 in llvm::CodeViewDebug::maybeRecordLocation(llvm::DebugLoc const&, llvm::MachineFunction const*) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#8  0x00007f3e04e6acff in llvm::CodeViewDebug::beginInstruction(llvm::MachineInstr const*) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#9  0x00007f3e04e014e9 in llvm::AsmPrinter::EmitFunctionBody() ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#10 0x00007f3e044e08ac in llvm::X86AsmPrinter::runOnMachineFunction(llvm::MachineFunction&) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#11 0x00007f3e04fb4b65 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#12 0x00007f3e057a2f63 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#13 0x00007f3e057a331b in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#14 0x00007f3e057a3651 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
#15 0x00007f3e044ad564 in LLVMRustWriteOutputFile ()
   from /opt/rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-d57d2b4ceb3a380c.so
