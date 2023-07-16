
rustc +stage2 -C lto -C save-temps main.rs

# Last .bc file is "main.main.7rcbfp3g-cgu.6.rcgu.bc"

build/aarch64-apple-darwin/llvm/bin/llc main.main.7rcbfp3g-cgu.6.rcgu.bc -filetype=obj -mcpu=apple-a12 -march=arm64 -O0
Assertion failed: (TmpVec.size() > 1), function buildUnmerge, file src/llvm-project/llvm/lib/CodeGen/GlobalISel/MachineIRBuilder.cpp, line 577.
PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace.
Stack dump:
0.      Program arguments: build/aarch64-apple-darwin/llvm/bin/llc main.main.7rcbfp3g-cgu.6.rcgu.bc -filetype=obj -mcpu=apple-a12 -march=arm64 -O0
1.      Running pass 'Function Pass Manager' on module 'main.main.7rcbfp3g-cgu.6.rcgu.bc'.
2.      Running pass 'Legalizer' on function '@_ZN3std2io5stdio8print_to17h83af8c48359573cfE'
0  llc                      0x00000001041729a8 llvm::sys::PrintStackTrace(llvm::raw_ostream&) + 52
1  llc                      0x0000000104171b08 llvm::sys::RunSignalHandlers() + 128
2  llc                      0x0000000104172fa0 SignalHandler(int) + 292
3  libsystem_platform.dylib 0x00000001a8d1cc44 _sigtramp + 56
4  libsystem_pthread.dylib  0x00000001a8cd4c24 pthread_kill + 292
5  libsystem_c.dylib        0x00000001a8c1c864 abort + 104
6  libsystem_c.dylib        0x00000001a8c1bb68 err + 0
7  llc                      0x0000000104f20790 llvm::MachineIRBuilder::buildBuildVector(llvm::DstOp const&, llvm::ArrayRef<llvm::Register>) (.cold.1) + 0
8  llc                      0x0000000104498130 llvm::MachineIRBuilder::buildBuildVector(llvm::DstOp const&, llvm::ArrayRef<llvm::Register>) + 0
9  llc                      0x0000000104475518 llvm::LegalizerHelper::extractParts(llvm::Register, llvm::LLT, int, llvm::SmallVectorImpl<llvm::Register>&) + 204
10 llc                      0x000000010446fd88 llvm::LegalizerHelper::narrowScalar(llvm::MachineInstr&, unsigned int, llvm::LLT) + 4184
11 llc                      0x0000000104466bb0 llvm::Legalizer::legalizeMachineFunction(llvm::MachineFunction&, llvm::LegalizerInfo const&, llvm::ArrayRef<llvm::GISelChangeObserver*>, llvm::LostDebugLocObserver&, llvm::MachineIRBuilder&) + 1736
12 llc                      0x0000000104468244 llvm::Legalizer::runOnMachineFunction(llvm::MachineFunction&) + 1104
13 llc                      0x00000001038a92f4 llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 348
14 llc                      0x0000000103bae7c0 llvm::FPPassManager::runOnFunction(llvm::Function&) + 744
15 llc                      0x0000000103bb3954 llvm::FPPassManager::runOnModule(llvm::Module&) + 68
16 llc                      0x0000000103baed80 llvm::legacy::PassManagerImpl::run(llvm::Module&) + 972
17 llc                      0x0000000102a644b8 compileModule(char**, llvm::LLVMContext&) + 5620
18 llc                      0x0000000102a62918 main + 1296
19 libdyld.dylib            0x00000001a8cf0f54 start + 4
