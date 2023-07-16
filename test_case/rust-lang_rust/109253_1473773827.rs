plain
[2632/3025] Linking CXX static library lib\libLLVMWebAssemblyDisassembler.a
[2633/3025] Linking CXX static library lib\libLLVMPowerPCCodeGen.a
[2634/3025] Linking CXX static library lib\libLLVMSystemZCodeGen.a
[2635/3025] Linking CXX static library lib\libLLVMM68kCodeGen.a
FAILED: lib/libLLVMM68kCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMM68kCodeGen.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMM68kCodeGen.a  lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kCallLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kInstructionSelector.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kLegalizerInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GISel/M68kRegisterBankInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kAsmPrinter.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kCollapseMOVEMPass.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kExpandPseudo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kFrameLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kInstrInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelLowering.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelDAGToDAG.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMachineFunction.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMCInstLower.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kRegisterInfo.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kSubtarget.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetMachine.cpp.obj lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetObjectFile.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMM68kCodeGen.a && cd ."
D:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2636/3025] Linking CXX static library lib\libLLVMAVRCodeGen.a
[2637/3025] Linking CXX static library lib\libLLVMWebAssemblyCodeGen.a
[2638/3025] Building CXX object tools/lli/ChildTarget/CMakeFiles/lli-child-target.dir/ChildTarget.cpp.obj
[2639/3025] Linking CXX static library lib\libLLVMX86CodeGen.a
[2639/3025] Linking CXX static library lib\libLLVMX86CodeGen.a
[2640/3025] Linking CXX executable bin\llvm-profdata.exe
[2641/3025] Building CXX object tools/lli/CMakeFiles/lli.dir/ExecutionUtils.cpp.obj
[2642/3025] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 245.973 seconds
Build completed unsuccessfully in 0:07:00
