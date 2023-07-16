plain
[ 17%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionDXContainer.cpp.o
[ 17%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionELF.cpp.o
[ 17%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionMachO.cpp.o
[ 17%] Building PPCGenAsmMatcher.inc...
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
[ 17%] Building PPCGenCallingConv.inc...
[ 17%] Building PPCGenCallingConv.inc...
0. Program arguments: ../../../bin/llvm-tblgen -gen-global-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64/AArch64.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/AArch64/AArch64GenGlobalISel.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x0000000107d5eaaa llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x0000000107d5db98 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x0000000107d5f130 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
4  llvm-tblgen              0x0000000107b62b49 GenerateVariantsOf(std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>&, llvm::CodeGenDAGPatterns&, llvm::StringSet<llvm::MallocAllocator> const&) + 5913
5  llvm-tblgen              0x0000000107b65727 CombineChildVariants(std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::vector<std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>, std::__1::allocator<std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>>> const&, std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>&, llvm::CodeGenDAGPatterns&, llvm::StringSet<llvm::MallocAllocator> const&) + 999
6  llvm-tblgen              0x0000000107b61a44 GenerateVariantsOf(std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>&, llvm::CodeGenDAGPatterns&, llvm::StringSet<llvm::MallocAllocator> const&) + 1556
7  llvm-tblgen              0x0000000107b619b0 GenerateVariantsOf(std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>&, llvm::CodeGenDAGPatterns&, llvm::StringSet<llvm::MallocAllocator> const&) + 1408
8  llvm-tblgen              0x0000000107b619b0 GenerateVariantsOf(std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::vector<std::__1::shared_ptr<llvm::TreePatternNode>, std::__1::allocator<std::__1::shared_ptr<llvm::TreePatternNode>>>&, llvm::CodeGenDAGPatterns&, llvm::StringSet<llvm::MallocAllocator> const&) + 1408
9  llvm-tblgen              0x0000000107b59761 llvm::CodeGenDAGPatterns::GenerateVariants() + 449
10 llvm-tblgen              0x0000000107b566a5 llvm::CodeGenDAGPatterns::CodeGenDAGPatterns(llvm::RecordKeeper&, std::__1::function<void (llvm::TreePattern*)>) + 917
11 llvm-tblgen              0x0000000107c337c9 llvm::EmitGlobalISel(llvm::RecordKeeper&, llvm::raw_ostream&) + 73
12 llvm-tblgen              0x0000000107cfa4ce (anonymous namespace)::LLVMTableGenMain(llvm::raw_ostream&, llvm::RecordKeeper&) + 974
13 llvm-tblgen              0x0000000107d646c5 llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 901
14 llvm-tblgen              0x0000000107cfa0c9 main + 137
15 dyld                     0x0000000109f8f52e start + 462
/bin/sh: line 1: 79685 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-global-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64/AArch64.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/AArch64/AArch64GenGlobalISel.inc
make[2]: *** [lib/Target/AArch64/AArch64GenGlobalISel.inc] Error 139
make[2]: *** Waiting for unfinished jobs....
[ 18%] Building ARMGenRegisterBank.inc...
[ 19%] Building PPCGenDisassemblerTables.inc...
[ 19%] Building MipsGenRegisterInfo.inc...
[ 19%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionWasm.cpp.o
---
[ 20%] Building PPCGenGlobalISel.inc...
[ 20%] Building RISCVGenMCCodeEmitter.inc...
[ 20%] Building SparcGenRegisterInfo.inc...
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinCOFFStreamer.cpp.o
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinEH.cpp.o
0. Program arguments: ../../../bin/llvm-tblgen -gen-emitter -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenMCCodeEmitter.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCXCOFFObjectTargetWriter.cpp.o
0  llvm-tblgen              0x000000010f315aaa llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010f314b98 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x000000010f316130 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
4  libsystem_platform.dylib 0x00007fd269704a58 _sigtramp + 18446743912018353272
5  llvm-tblgen              0x000000010f3247ea llvm::BinOpInit::resolveReferences(llvm::Resolver&) const + 42
6  llvm-tblgen              0x000000010f3247ea llvm::BinOpInit::resolveReferences(llvm::Resolver&) const + 42
7  llvm-tblgen              0x000000010f3247ea llvm::BinOpInit::resolveReferences(llvm::Resolver&) const + 42
8  llvm-tblgen              0x000000010f32ab35 llvm::Record::resolveReferences(llvm::Resolver&, llvm::RecordVal const*) + 149
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MachObjectWriter.cpp.o
9  llvm-tblgen              0x000000010f3414d6 llvm::TGParser::resolve(std::__1::vector<llvm::RecordsEntry, std::__1::allocator<llvm::RecordsEntry>> const&, llvm::SmallVector<std::__1::pair<llvm::Init*, llvm::Init*>, 8u>&, bool, std::__1::vector<llvm::RecordsEntry, std::__1::allocator<llvm::RecordsEntry>>*, llvm::SMLoc*) + 1030
10 llvm-tblgen              0x000000010f34fea8 llvm::TGParser::ParseDefm(llvm::MultiClass*) + 1416
11 llvm-tblgen              0x000000010f34d117 llvm::TGParser::ParseForeach(llvm::MultiClass*) + 503
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/SPIRVObjectWriter.cpp.o
12 llvm-tblgen              0x000000010f34cac1 llvm::TGParser::ParseDefset() + 593
13 llvm-tblgen              0x000000010f350ad2 llvm::TGParser::ParseFile() + 82
14 llvm-tblgen              0x000000010f31b60a llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 714
15 llvm-tblgen              0x000000010f2b10c9 main + 137
16 dyld                     0x0000000116c1652e start + 462
/bin/sh: line 1: 80197 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-emitter -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenMCCodeEmitter.inc
make[2]: *** [lib/Target/RISCV/RISCVGenMCCodeEmitter.inc] Error 139
make[2]: *** Waiting for unfinished jobs....
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/StringTableBuilder.cpp.o
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/TargetRegistry.cpp.o
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/TargetRegistry.cpp.o
0. Program arguments: ../../../bin/llvm-tblgen -gen-disassembler -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenDisassemblerTables.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x000000010b93faaa llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010b93eb98 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x000000010b940130 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
4  llvm-tblgen              0x000000010b90ff1b llvm::FoldingSetNodeID::operator==(llvm::FoldingSetNodeID const&) const + 27
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/WinCOFFObjectWriter.cpp.o
5  llvm-tblgen              0x000000010b954f3f llvm::Record::resolveReferences(llvm::Init*) + 95
[ 20%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/XCOFFObjectWriter.cpp.o
6  llvm-tblgen              0x000000010b96c097 llvm::TGParser::addDefOne(std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>) + 199
7  llvm-tblgen              0x000000010b96b825 llvm::TGParser::addEntry(llvm::RecordsEntry) + 293
8  llvm-tblgen              0x000000010b97a8de llvm::TGParser::ParseDefm(llvm::MultiClass*) + 4030
9  llvm-tblgen              0x000000010b978cd7 llvm::TGParser::ParseTopLevelLet(llvm::MultiClass*) + 1111
10 llvm-tblgen              0x000000010b97aad2 llvm::TGParser::ParseFile() + 82
11 llvm-tblgen              0x000000010b94560a llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 714
12 llvm-tblgen              0x000000010b8db0c9 main + 137
13 dyld                     0x000000011372f52e start + 462
/bin/sh: line 1: 80066 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-disassembler -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenDisassemblerTables.inc
make[2]: *** [lib/Target/RISCV/RISCVGenDisassemblerTables.inc] Error 139
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-register-info -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/Sparc -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/Sparc/Sparc.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/Sparc/SparcGenRegisterInfo.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x000000010997caaa llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010997bb98 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x000000010997d130 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
4  llvm-tblgen              0x000000010999c033 std::__1::pair<llvm::StringMapIterator<llvm::StringInit*>, bool> llvm::StringMap<llvm::StringInit*, llvm::BumpPtrAllocatorImpl<llvm::MallocAllocator, 4096ul, 4096ul, 128ul>&>::try_emplace<llvm::StringInit*>(llvm::StringRef, llvm::StringInit*&&) + 35
5  llvm-tblgen              0x0000000109990014 llvm::TypedInit::getCastTo(llvm::RecTy*) const + 52
6  llvm-tblgen              0x0000000109994002 llvm::RecordVal::setValue(llvm::Init*) + 66
7  llvm-tblgen              0x0000000109991b43 llvm::Record::resolveReferences(llvm::Resolver&, llvm::RecordVal const*) + 163
8  llvm-tblgen              0x0000000109991f3f llvm::Record::resolveReferences(llvm::Init*) + 95
9  llvm-tblgen              0x00000001099a9097 llvm::TGParser::addDefOne(std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>) + 199
10 llvm-tblgen              0x00000001099a8825 llvm::TGParser::addEntry(llvm::RecordsEntry) + 293
[ 21%] Linking CXX static library ../libLLVMMC.a
11 llvm-tblgen              0x00000001099b78de llvm::TGParser::ParseDefm(llvm::MultiClass*) + 4030
12 llvm-tblgen              0x00000001099b7ad2 llvm::TGParser::ParseFile() + 82
13 llvm-tblgen              0x000000010998260a llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 714
14 llvm-tblgen              0x00000001099180c9 main + 137
15 dyld                     0x0000000113f7a52e start + 462
/bin/sh: line 1: 80200 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-register-info -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/Sparc -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/Sparc/Sparc.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/Sparc/SparcGenRegisterInfo.inc
make[2]: *** [lib/Target/Sparc/SparcGenRegisterInfo.inc] Error 139
make[1]: *** [lib/Target/Sparc/CMakeFiles/SparcCommonTableGen.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 21%] Built target PowerPCCommonTableGen
make[1]: *** [lib/Target/AArch64/CMakeFiles/AArch64CommonTableGen.dir/all] Error 2
make[1]: *** [lib/Target/AArch64/CMakeFiles/AArch64CommonTableGen.dir/all] Error 2
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-dag-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -omit-comments -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenDAGISel.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x0000000109abfaaa llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x0000000109abeb98 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x0000000109ac0130 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff80efc2dfd _sigtramp + 29
4  llvm-tblgen              0x0000000109b31249
5  llvm-tblgen              0x00000001098b1db2 llvm::TreePatternNode::NodeHasProperty(llvm::SDNP, llvm::CodeGenDAGPatterns const&) const + 274
6  llvm-tblgen              0x00000001098c598d InstAnalyzer::AnalyzeNode(llvm::TreePatternNode const*) + 317
7  llvm-tblgen              0x00000001098bba1a llvm::CodeGenDAGPatterns::InferInstructionFlags() + 442
8  llvm-tblgen              0x00000001098b76b5 llvm::CodeGenDAGPatterns::CodeGenDAGPatterns(llvm::RecordKeeper&, std::__1::function<void (llvm::TreePattern*)>) + 933
9  llvm-tblgen              0x00000001099203be llvm::EmitDAGISel(llvm::RecordKeeper&, llvm::raw_ostream&) + 94
10 llvm-tblgen              0x0000000109a5b212 (anonymous namespace)::LLVMTableGenMain(llvm::raw_ostream&, llvm::RecordKeeper&) + 274
11 llvm-tblgen              0x0000000109ac56c5 llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 901
12 llvm-tblgen              0x0000000109a5b0c9 main + 137
13 dyld                     0x00000001181e252e start + 462
/bin/sh: line 1: 80048 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-dag-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -omit-comments -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenDAGISel.inc
make[2]: *** [lib/Target/RISCV/RISCVGenDAGISel.inc] Error 139
make[1]: *** [lib/Target/RISCV/CMakeFiles/RISCVCommonTableGen.dir/all] Error 2
make: *** [all] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
 finished in 102.891 seconds
Build completed unsuccessfully in 0:03:13
