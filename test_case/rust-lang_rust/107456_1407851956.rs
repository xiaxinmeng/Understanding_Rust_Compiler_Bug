plain
[183/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/Attributes.cpp.obj
[184/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenSchedule.cpp.obj
In file included from D:/a/rust/rust/src/llvm-project/llvm/utils/TableGen/CodeGenSchedule.h:17,
                 from D:/a/rust/rust/src/llvm-project/llvm/utils/TableGen/CodeGenSchedule.cpp:14:
In member function 'llvm::APInt& llvm::APInt::operator=(llvm::APInt&&)',
    inlined from 'std::pair<_T1, _T2>& std::pair<_T1, _T2>::operator=(std::__conditional_t<std::__and_<std::is_move_assignable<_Tp>, std::is_move_assignable<_T2> >::value, std::pair<_T1, _T2>&&, std::__nonesuch&&>) [with _T1 = llvm::APInt; _T2 = llvm::APInt]' at D:/a/rust/rust/mingw32/lib/gcc/i686-w64-mingw32/12.2.0/include/c++/bits/stl_pair.h:584:8,
    inlined from 'void processSTIPredicate(llvm::STIPredicateFunction&, const llvm::ProcModelMapTy&)' at D:/a/rust/rust/src/llvm-project/llvm/utils/TableGen/CodeGenSchedule.cpp:324:63,
    inlined from 'void llvm::CodeGenSchedModels::collectSTIPredicates()' at D:/a/rust/rust/src/llvm-project/llvm/utils/TableGen/CodeGenSchedule.cpp:421:24:
D:/a/rust/rust/src/llvm-project/llvm/include/llvm/ADT/APInt.h:625:11: warning: 'void* memcpy(void*, const void*, size_t)' accessing 8 bytes at offsets 0 and 0 overlaps 8 bytes at offset 0 [-Wrestrict]
  625 |     memcpy(&U, &that.U, sizeof(U));
[185/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenDAGPatterns.cpp.obj
[186/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenHwModes.cpp.obj
[187/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DFAEmitter.cpp.obj
[188/3021] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelEmitter.cpp.obj
---
[2196/3021] Building CXX object lib/Target/WebAssembly/AsmParser/CMakeFiles/LLVMWebAssemblyAsmParser.dir/WebAssemblyAsmTypeCheck.cpp.obj
[2197/3021] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVISelLowering.cpp.obj
In function 'llvm::SDValue combineORToSHFL(llvm::SDValue, llvm::SelectionDAG&, const llvm::RISCVSubtarget&)',
    inlined from 'llvm::SDValue performORCombine(llvm::SDNode*, llvm::SelectionDAG&, const llvm::RISCVSubtarget&)' at D:/a/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVISelLowering.cpp:8139:36:
D:/a/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVISelLowering.cpp:7796:43: warning: array subscript 4294967295 is above array bounds of 'const uint64_t [5]' {aka 'const long long unsigned int [5]'} [-Warray-bounds]
 7796 |   uint64_t ExpMask = BitmanipMasks[MaskIdx] & maskTrailingOnes<uint64_t>(Width);
D:/a/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVISelLowering.cpp: In function 'llvm::SDValue performORCombine(llvm::SDNode*, llvm::SelectionDAG&, const llvm::RISCVSubtarget&)':
D:/a/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCVISelLowering.cpp:7789:25: note: while referencing 'BitmanipMasks'
 7789 |   static const uint64_t BitmanipMasks[] = {
      |                         ^~~~~~~~~~~~~
---
[2598/3021] Building CXX object tools/lli/CMakeFiles/lli.dir/ExecutionUtils.cpp.obj
[2599/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachODebugMapParser.cpp.obj
[2600/3021] Linking CXX static library lib\libLLVMSelectionDAG.a
[2601/3021] Linking CXX static library lib\libLLVMGlobalISel.a
FAILED: lib/libLLVMGlobalISel.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMGlobalISel.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMGlobalISel.a  lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CSEInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GISelKnownBits.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CSEMIRBuilder.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CallLowering.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GlobalISel.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Combiner.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CombinerHelper.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GISelChangeObserver.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/IRTranslator.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InlineAsmLowering.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InstructionSelect.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InstructionSelector.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalityPredicates.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizeMutations.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Legalizer.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizerHelper.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizerInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegacyLegalizerInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LoadStoreOpt.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Localizer.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LostDebugLocObserver.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/MachineIRBuilder.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/RegBankSelect.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Utils.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMGlobalISel.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2602/3021] Linking CXX static library lib\libLLVMMIRParser.a
[2603/3021] Linking CXX static library lib\libLLVMAsmPrinter.a
[2604/3021] Building CXX object tools/lli/CMakeFiles/lli.dir/lli.cpp.obj
[2605/3021] Building CXX object tools/llc/CMakeFiles/llc.dir/llc.cpp.obj
[2605/3021] Building CXX object tools/llc/CMakeFiles/llc.dir/llc.cpp.obj
[2606/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil.cpp.obj
[2607/3021] Linking CXX executable bin\llvm-profdata.exe
[2608/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\cmake-0.1.48\src\lib.rs:975:5
 finished in 295.366 seconds
Build completed unsuccessfully in 0:06:49
Build completed unsuccessfully in 0:06:49
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
