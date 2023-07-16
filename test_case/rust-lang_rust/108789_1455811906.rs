plain
[2609/3021] Linking CXX static library lib\libLLVMHexagonCodeGen.a
[2610/3021] Linking CXX static library lib\libLLVMAArch64CodeGen.a
[2611/3021] Linking CXX static library lib\libLLVMWebAssemblyAsmParser.a
[2612/3021] Linking CXX static library lib\libLLVMSparcCodeGen.a
FAILED: lib/libLLVMSparcCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMSparcCodeGen.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMSparcCodeGen.a  lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/DelaySlotFiller.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/LeonPasses.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcAsmPrinter.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcInstrInfo.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcISelDAGToDAG.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcISelLowering.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcFrameLowering.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcMachineFunctionInfo.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcRegisterInfo.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcSubtarget.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcTargetMachine.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcMCInstLower.cpp.obj lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcTargetObjectFile.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMSparcCodeGen.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2613/3021] Linking CXX static library lib\libLLVMPowerPCCodeGen.a
[2614/3021] Linking CXX static library lib\libLLVMWebAssemblyDesc.a
[2615/3021] Linking CXX static library lib\libLLVMSystemZCodeGen.a
[2616/3021] Linking CXX static library lib\libLLVMRISCVCodeGen.a
[2616/3021] Linking CXX static library lib\libLLVMRISCVCodeGen.a
[2617/3021] Linking CXX static library lib\libLLVMX86CodeGen.a
[2618/3021] Linking CXX executable bin\llvm-profdata.exe
[2619/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 242.254 seconds
Build completed unsuccessfully in 0:05:25
Build completed unsuccessfully in 0:05:25
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
