plain
[2605/3021] Linking CXX static library lib\libLLVMSparcCodeGen.a
[2606/3021] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2607/3021] Linking CXX static library lib\libLLVMPowerPCCodeGen.a
[2608/3021] Linking CXX static library lib\libLLVMAVRCodeGen.a
FAILED: lib/libLLVMAVRCodeGen.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMAVRCodeGen.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMAVRCodeGen.a  lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRAsmPrinter.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRExpandPseudoInsts.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRFrameLowering.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRInstrInfo.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRISelDAGToDAG.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRISelLowering.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRMCInstLower.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRRegisterInfo.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRShiftExpand.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRSubtarget.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRTargetMachine.cpp.obj lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRTargetObjectFile.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMAVRCodeGen.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2609/3021] Linking CXX static library lib\libLLVMWebAssemblyAsmParser.a
[2610/3021] Linking CXX static library lib\libLLVMWebAssemblyDesc.a
[2611/3021] Linking CXX static library lib\libLLVMSystemZCodeGen.a
[2612/3021] Linking CXX static library lib\libLLVMM68kCodeGen.a
[2612/3021] Linking CXX static library lib\libLLVMM68kCodeGen.a
[2613/3021] Linking CXX static library lib\libLLVMX86CodeGen.a
[2614/3021] Linking CXX static library lib\libLLVMPasses.a
[2615/3021] Linking CXX executable bin\llvm-profdata.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 231.571 seconds
Build completed unsuccessfully in 0:06:33
