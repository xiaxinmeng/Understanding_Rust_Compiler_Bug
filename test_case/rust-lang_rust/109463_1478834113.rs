plain
[2598/3021] Linking CXX static library lib\libLLVMInterpreter.a
[2599/3021] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2600/3021] Linking CXX static library lib\libLLVMGlobalISel.a
FAILED: lib/libLLVMGlobalISel.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMGlobalISel.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMGlobalISel.a  lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CSEInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GISelKnownBits.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CSEMIRBuilder.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CallLowering.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GlobalISel.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Combiner.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CombinerHelper.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GISelChangeObserver.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/IRTranslator.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InlineAsmLowering.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InstructionSelect.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/InstructionSelector.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalityPredicates.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizeMutations.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Legalizer.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizerHelper.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegalizerInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LegacyLegalizerInfo.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LoadStoreOpt.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Localizer.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/LostDebugLocObserver.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/MachineIRBuilder.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/RegBankSelect.cpp.obj lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Utils.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMGlobalISel.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2601/3021] Linking CXX static library lib\libLLVMDWARFLinker.a
[2602/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ToolRunner.cpp.obj
[2603/3021] Linking CXX static library lib\libLLVMBPFCodeGen.a
[2604/3021] Linking CXX static library lib\libLLVMHexagonCodeGen.a
[2604/3021] Linking CXX static library lib\libLLVMHexagonCodeGen.a
[2605/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/bugpoint.cpp.obj
[2606/3021] Linking CXX executable bin\llvm-profdata.exe
[2607/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 301.361 seconds
Build completed unsuccessfully in 0:06:16
Build completed unsuccessfully in 0:06:16
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
