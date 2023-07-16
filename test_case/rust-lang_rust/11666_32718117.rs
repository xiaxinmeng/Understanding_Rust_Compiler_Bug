
[~/code/rust2[master]] $ ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llc -mtriple=arm-apple-darwin ./src/rt/rust_try.ll -filetype=obj -arm-enable-ehabi -arm-enable-ehabi-descriptors     [alex@rhea] │
0  llc                      0x000000010526d248 llvm::sys::PrintStackTrace(__sFILE*) + 40                                                                                                                 │
1  llc                      0x000000010526d744 SignalHandler(int) + 548                                                                                                                                  │
2  libsystem_platform.dylib 0x00007fff889795aa _sigtramp + 26                                                                                                                                            │
3  llc                      0x000000010525e488 llvm::FoldingSetImpl::FindNodeOrInsertPos(llvm::FoldingSetNodeID const&, void*&) + 72                                                                     │
4  llc                      0x0000000104a7bbd3 llvm::ARMAsmPrinter::EmitInstruction(llvm::MachineInstr const*) + 131                                                                                     │
5  llc                      0x0000000104dc1459 llvm::AsmPrinter::EmitFunctionBody() + 3657                                                                                                               │
6  llc                      0x0000000104a74758 llvm::ARMAsmPrinter::runOnMachineFunction(llvm::MachineFunction&) + 264                                                                                   │
7  llc                      0x0000000104e9055d llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 125                                                                                           │
8  llc                      0x000000010516d21b llvm::FPPassManager::runOnFunction(llvm::Function&) + 347                                                                                                 │
9  llc                      0x000000010516d4ab llvm::FPPassManager::runOnModule(llvm::Module&) + 43                                                                                                      │
10 llc                      0x000000010516da89 llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1081                                                                                                  │
11 llc                      0x000000010516de9d llvm::legacy::PassManager::run(llvm::Module&) + 13                                                                                                        │
12 llc                      0x000000010493518a main + 5002                                                                                                                                               │
13 libdyld.dylib            0x00007fff8dee55fd start + 1                                                                                                                                                 │
14 libdyld.dylib            0x0000000000000006 start + 1913760266                                                                                                                                        │
Stack dump:                                                                                                                                                                                              │
0.      Program arguments: ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llc -mtriple=arm-apple-darwin ./src/rt/rust_try.ll -filetype=obj -arm-enable-ehabi -arm-enable-ehabi-descriptors               │
1.      Running pass 'Function Pass Manager' on module './src/rt/rust_try.ll'.                                                                                                                           │
2.      Running pass 'ARM Assembly / Object Emitter' on function '@rust_try'                                                                                                                             │
zsh: segmentation fault  ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llc -mtriple=arm-apple-darwin                                                                                                    │
