plain
[2605/3021] Linking CXX static library lib\libLLVMARMCodeGen.a
[2606/3021] Linking CXX static library lib\libLLVMPowerPCCodeGen.a
[2607/3021] Linking CXX static library lib\libLLVMAArch64CodeGen.a
[2608/3021] Linking CXX static library lib\libLLVMSystemZCodeGen.a
FAILED: lib/libLLVMSystemZCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMSystemZCodeGen.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMSystemZCodeGen.a  lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZAsmPrinter.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZCallingConv.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZConstantPoolValue.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZCopyPhysRegs.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZElimCompare.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZFrameLowering.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZHazardRecognizer.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelDAGToDAG.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelLowering.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZInstrInfo.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZLDCleanup.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZLongBranch.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZMachineFunctionInfo.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZMachineScheduler.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZMCInstLower.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZPostRewrite.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZRegisterInfo.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZSelectionDAGInfo.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZShortenInst.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZSubtarget.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZTargetMachine.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZTargetTransformInfo.cpp.obj lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZTDC.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMSystemZCodeGen.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2609/3021] Linking CXX static library lib\libLLVMWebAssemblyAsmParser.a
[2610/3021] Linking CXX static library lib\libLLVMRISCVCodeGen.a
[2611/3021] Linking CXX static library lib\libLLVMWebAssemblyDesc.a
[2612/3021] Linking CXX static library lib\libLLVMSparcCodeGen.a
[2612/3021] Linking CXX static library lib\libLLVMSparcCodeGen.a
[2613/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/bugpoint.cpp.obj
[2614/3021] Linking CXX executable bin\llvm-profdata.exe
[2615/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 232.957 seconds
Build completed unsuccessfully in 0:06:35
