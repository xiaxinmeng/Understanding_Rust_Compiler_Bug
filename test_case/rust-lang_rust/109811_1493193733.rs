plain
[ 13%] Building ARMGenCallingConv.inc...
[ 13%] Building ARMGenDAGISel.inc...
[ 13%] Building BPFGenSubtargetInfo.inc...
[ 13%] Built target MSP430CommonTableGen
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
[ 13%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStringTable.cpp.o
[ 13%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStringTable.cpp.o
0. Program arguments: ../../../bin/llvm-tblgen -gen-dag-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/NVPTX -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -omit-comments -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/NVPTX/NVPTX.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/NVPTX/NVPTXGenDAGISel.inc
[ 13%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkParser.cpp.o
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x000000010a3a66ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010a3a5868 llvm::sys::RunSignalHandlers() + 248
[ 13%] Building RISCVGenAsmWriter.inc...
2  llvm-tblgen              0x000000010a3a6d50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff81f5d6dfd _sigtramp + 29
[ 13%] Building RISCVGenCompressInstEmitter.inc...
[ 13%] Building RISCVGenCompressInstEmitter.inc...
4  libsystem_platform.dylib 0x000000010a5fa000 _sigtramp + 18446603374523724320
5  libsystem_c.dylib        0x00007ff81f50cd24 abort + 123
5  libsystem_c.dylib        0x00007ff81f50cd24 abort + 123
6  libsystem_malloc.dylib   0x00007ff81f3ea357 has_default_zone0 + 0
7  libsystem_malloc.dylib   0x00007ff81f3fe308 malloc_zone_error + 178
[ 13%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkSerializer.cpp.o
8  libsystem_malloc.dylib   0x00007ff81f3e23e4 tiny_free_list_remove_ptr + 698
9  libsystem_malloc.dylib   0x00007ff81f3e1641 tiny_free_no_lock + 804
10 libsystem_malloc.dylib   0x00007ff81f3e11c4 free_tiny + 446
11 llvm-tblgen              0x000000010a1abb74 llvm::TreePattern::~TreePattern() + 244
[ 13%] Building RISCVGenDisassemblerTables.inc...
12 llvm-tblgen              0x000000010a1a6acb llvm::CodeGenDAGPatterns::ParseOnePattern(llvm::Record*, llvm::TreePattern&, llvm::TreePattern&, std::__1::vector<llvm::Record*, std::__1::allocator<llvm::Record*>> const&) + 2043
13 llvm-tblgen              0x000000010a1a047e llvm::CodeGenDAGPatterns::ParseInstructions() + 3342
14 llvm-tblgen              0x000000010a19db78 llvm::CodeGenDAGPatterns::CodeGenDAGPatterns(llvm::RecordKeeper&, std::__1::function<void (llvm::TreePattern*)>) + 888
15 llvm-tblgen              0x000000010a203cbe llvm::EmitDAGISel(llvm::RecordKeeper&, llvm::raw_ostream&) + 94
16 llvm-tblgen              0x000000010a343e62 (anonymous namespace)::LLVMTableGenMain(llvm::raw_ostream&, llvm::RecordKeeper&) + 274
17 llvm-tblgen              0x000000010a3ac2dc llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 908
18 llvm-tblgen              0x000000010a343d19 main + 137
19 dyld                     0x000000011032952e start + 462
/bin/sh: line 1: 30737 Abort trap: 6           ../../../bin/llvm-tblgen -gen-dag-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/NVPTX -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -omit-comments -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/NVPTX/NVPTX.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/NVPTX/NVPTXGenDAGISel.inc
make[2]: *** [lib/Target/NVPTX/NVPTXGenDAGISel.inc] Error 134
make[2]: *** Waiting for unfinished jobs....
[ 13%] Built target LLVMRemarks
[ 13%] Building PPCGenDisassemblerTables.inc...
[ 13%] Building MipsGenDisassemblerTables.inc...
[ 13%] Building MipsGenFastISel.inc...
[ 13%] Building MipsGenFastISel.inc...
[ 13%] Building AArch64GenAsmWriter1.inc...
make[1]: *** [lib/Target/NVPTX/CMakeFiles/NVPTXCommonTableGen.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 13%] Building AArch64GenDAGISel.inc...
[ 13%] Building HexagonGenDFAPacketizer.inc...
[ 13%] Building AArch64GenDisassemblerTables.inc...
[ 13%] Building PPCGenFastISel.inc...
---
[ 16%] Building AArch64GenPostLegalizeGILowering.inc...
[ 16%] Built target ARMCommonTableGen
[ 16%] Building AArch64GenPreLegalizeGICombiner.inc...
[ 16%] Building AArch64GenRegisterBank.inc...
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td:30:
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVInstrInfo.td:1857:
/Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVInstrInfoV.td:1710:11: error: Record `VLOXSEG7EI8_V' does not have a field named `isConvergent'!


      def VLOXSEG#nf#EI#eew#_V :
          ^
make[2]: *** [lib/Target/RISCV/RISCVGenSubtargetInfo.inc] Error 1
make[2]: *** Waiting for unfinished jobs....
[ 16%] Building AArch64GenSystemOperands.inc...
[ 16%] Built target AArch64CommonTableGen
make[1]: *** [lib/Target/RISCV/CMakeFiles/RISCVCommonTableGen.dir/all] Error 2
make: *** [all] Error 2
make: *** [all] Error 2
thread 'main' panicked at '
command did not execute successfully, got: exit status: 2

build script failed, must exit now', /Users/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
 finished in 125.073 seconds
Build completed unsuccessfully in 0:03:37
