plain
[2611/3021] Building CXX object tools/llc/CMakeFiles/llc.dir/llc.cpp.obj
[2612/3021] Linking CXX static library lib\libLLVMNVPTXCodeGen.a
[2613/3021] Linking CXX static library lib\libLLVMMipsCodeGen.a
[2614/3021] Linking CXX static library lib\libLLVMRISCVCodeGen.a
FAILED: lib/libLLVMRISCVCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMRISCVCodeGen.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMRISCVCodeGen.a  lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVAsmPrinter.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVCallLowering.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVCodeGenPrepare.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMakeCompressible.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVExpandAtomicPseudoInsts.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVExpandPseudoInsts.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVFrameLowering.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVGatherScatterLowering.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVInsertVSETVLI.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVInstrInfo.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVInstructionSelector.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVISelDAGToDAG.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVISelLowering.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVLegalizerInfo.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMachineFunctionInfo.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMacroFusion.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMCInstLower.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMergeBaseOffset.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVRedundantCopyElimination.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVRegisterBankInfo.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVRegisterInfo.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVSExtWRemoval.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVSubtarget.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVTargetMachine.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVTargetObjectFile.cpp.obj lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVTargetTransformInfo.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMRISCVCodeGen.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2615/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/CFBundle.cpp.obj
[2616/3021] Linking CXX static library lib\libLLVMPowerPCCodeGen.a
[2617/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/Reproducer.cpp.obj
[2618/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/SymbolMap.cpp.obj
[2618/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/SymbolMap.cpp.obj
[2619/3021] Linking CXX executable bin\llvm-profdata.exe
[2620/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DwarfLinkerForBinary.cpp.obj
[2621/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 295.029 seconds
Build completed unsuccessfully in 0:07:49
