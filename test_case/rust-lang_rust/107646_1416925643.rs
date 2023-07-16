plain
[2615/3021] Linking CXX static library lib\libLLVMWebAssemblyDesc.a
[2616/3021] Building Options.inc...
[2617/3021] Linking CXX static library lib\libLLVMSystemZCodeGen.a
[2618/3021] Linking CXX static library lib\libLLVMM68kCodeGen.a
FAILED: lib/libLLVMM68kCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMM68kCodeGen.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMM68kCodeGen.a  lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kCallLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kInstructionSelector.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kLegalizerInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kRegisterBankInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kAsmPrinter.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kCollapseMOVEMPass.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kExpandPseudo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kFrameLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kInstrInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelDAGToDAG.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMachineFunction.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMCInstLower.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kRegisterInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kSubtarget.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetMachine.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetObjectFile.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMM68kCodeGen.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2619/3021] Linking CXX static library lib\libLLVMAVRCodeGen.a
[2620/3021] Linking CXX static library lib\libLLVMWebAssemblyDisassembler.a
[2621/3021] Linking CXX static library lib\libLLVMWebAssemblyCodeGen.a
[2622/3021] Linking CXX static library lib\libLLVMX86CodeGen.a
[2622/3021] Linking CXX static library lib\libLLVMX86CodeGen.a
[2623/3021] Building CXX object tools/llc/CMakeFiles/llc.dir/llc.cpp.obj
[2624/3021] Linking CXX executable bin\llvm-profdata.exe
[2625/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 262.035 seconds
Build completed successfully in 0:07:42
