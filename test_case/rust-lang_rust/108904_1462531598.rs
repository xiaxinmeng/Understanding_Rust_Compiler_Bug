plain
[2607/3021] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2608/3021] Linking CXX static library lib\libLLVMInterpreter.a
[2609/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil.cpp.obj
[2610/3021] Linking CXX static library lib\libLLVMMSP430CodeGen.a
FAILED: lib/libLLVMMSP430CodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMMSP430CodeGen.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMMSP430CodeGen.a  lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430BranchSelector.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430ISelDAGToDAG.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430ISelLowering.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430InstrInfo.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430FrameLowering.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430MachineFunctionInfo.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430RegisterInfo.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430Subtarget.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430TargetMachine.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430AsmPrinter.cpp.obj lib/Target/MSP430/CMakeFiles/LLVMMSP430CodeGen.dir/MSP430MCInstLower.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMMSP430CodeGen.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2611/3021] Linking CXX static library lib\libLLVMGlobalISel.a
[2612/3021] Linking CXX static library lib\libLLVMBPFCodeGen.a
[2613/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/CFBundle.cpp.obj
[2614/3021] Linking CXX static library lib\libLLVMNVPTXCodeGen.a
[2614/3021] Linking CXX static library lib\libLLVMNVPTXCodeGen.a
[2615/3021] Linking CXX static library lib\libLLVMHexagonCodeGen.a
[2616/3021] Linking CXX executable bin\llvm-profdata.exe
[2617/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 292.424 seconds
Build completed unsuccessfully in 0:06:33
Build completed unsuccessfully in 0:06:33
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
