plain
[00:18:09] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:18:09] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
[00:18:09] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:18:09] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
[00:18:09] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG
[00:18:09] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG - Failed
[00:18:09] -- Performing Test C_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG - Failed
[00:18:09] -- Performing Test CXX_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG
[00:18:09] -- Performing Test CXX_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG - Success
[00:18:09] -- Performing Test C_WCOMMENT_ALLOWS_LINE_WRAP
---
[00:18:13] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.o
[00:18:13] [  0%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[00:18:14] [  0%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.o
[00:18:14] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APFloat.cpp.o
[00:18:14] In file included from /checkout/src/llvm/lib/TableGen/JSONBackend.cpp:20:0:
[00:18:14] /checkout/src/llvm/include/llvm/Support/JSON.h:393:23:   required from here
[00:18:14] /checkout/src/llvm/include/llvm/Support/JSON.h:455:47: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
[00:18:14]      return *reinterpret_cast<T *>(Union.buffer);
[00:18:14]                                                ^
---
[00:20:52] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterInfo.cpp.o
[00:20:52] [ 67%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ShuffleDecodeConstantPool.cpp.o
[00:20:52] [ 67%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CallLowering.cpp.o
[00:20:53] [ 67%] Building CXX object lib/Target/AArch64/Disassembler/CMakeFiles/LLVMAArch64Disassembler.dir/AArch64ExternalSymbolizer.cpp.o
[00:20:53] [ 67%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeLoadHardening.cpp.o
[00:20:53] [ 67%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CleanupLocalDynamicTLSPass.cpp.o
[00:20:53] [ 67%] Linking CXX static library ../../../libLLVMAArch64Disassembler.a
[00:20:53] make[3]: Leaving directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:20:53] [ 67%] Built target LLVMAArch64Disassembler
---
[00:21:27] Scanning dependencies of target LLVMHexagonDisassembler
[00:21:27] make[3]: Leaving directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:21:27] make[3]: Entering directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:21:27] [ 81%] Building CXX object lib/Target/Hexagon/Disassembler/CMakeFiles/LLVMHexagonDisassembler.dir/HexagonDisassembler.cpp.o
[00:21:27] /checkout/src/llvm/lib/Target/Hexagon/HexagonBitSimplify.cpp: In function 'virtual bool {anonymous}::BitSimplification::processBlock(llvm::MachineBasicBlock&, const {anonymous}::RegisterSet&)':
[00:21:27] /checkout/src/llvm/lib/Target/Hexagon/HexagonBitSimplify.cpp:1989:16: warning: 'V' is used uninitialized in this function [-Wuninitialized]
[00:21:27]    if (!isInt<8>(V))
[00:21:27]                 ^
[00:21:27] /checkout/src/llvm/lib/Target/Hexagon/HexagonBitSimplify.cpp:1977:7: note: 'V' was declared here
[00:21:27]    int V;
[00:21:27] [ 81%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBlockRanges.cpp.o
[00:21:27] [ 81%] Building CXX object lib/Target/NVPTX/CMakeFiles/LLVMNVPTXCodeGen.dir/NVPTXRegisterInfo.cpp.o
[00:21:27] [ 81%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBranchRelaxation.cpp.o
[00:21:27] [ 81%] Linking CXX static library ../../../libLLVMHexagonDisassembler.a
---
[00:21:30] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonInstPrinter.cpp.o
[00:21:30] Scanning dependencies of target LLVMWebAssemblyCodeGen
[00:21:30] make[3]: Leaving directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:21:30] make[3]: Entering directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:21:30] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyAddMissingPrototypes.cpp.o
[00:21:30] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyArgumentMove.cpp.o
[00:21:30] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCAsmInfo.cpp.o
[00:21:30] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyAsmPrinter.cpp.o
[00:21:30] [ 82%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonFrameLowering.cpp.o
---
[00:22:17] Scanning dependencies of target llvm-undname
[00:22:17] make[3]: Leaving directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:22:17] make[3]: Entering directory '/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build'
[00:22:17] [ 98%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Stage.cpp.o
[00:22:17] [ 98%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[00:22:18] [ 99%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Support.cpp.o
[00:22:18] [ 99%] Linking CXX executable ../../bin/llvm-undname
[00:22:18] [ 99%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageSummaryInfo.cpp.o
[00:22:18] [ 99%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/SummaryView.cpp.o
---
[00:48:24] [RUSTC-TIMING] aho_corasick test:false 1.401
[00:48:25] [RUSTC-TIMING] tempfile test:false 2.306
[00:48:31] [RUSTC-TIMING] regex_syntax test:false 19.977

Broadcast message from root@travis-job-720e4a93-aacc-4595-9d5f-b97ef701e1ec
 (unknown) at 12:18 ...
The system is going down for power off NOW!
[00:48:41] Makefile:28: recipe for target 'all' failed
[00:48:41] Makefile:28: recipe for target 'all' failed
[00:48:41] Session terminated, terminating shell... ...terminated.
[00:48:41] make: *** [all] Terminated

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:11a57d62
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
