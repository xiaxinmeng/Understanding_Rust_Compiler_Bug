plain
[1899/3021] Linking CXX static library lib\libLLVMARMDesc.a
[1900/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFIRPeephole.cpp.obj
[1901/3021] Linking CXX static library lib\libLLVMARMDisassembler.a
FAILED: lib/libLLVMARMDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMARMDisassembler.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMARMDisassembler.a  lib/Target/ARM/Disassembler/CMakeFiles/LLVMARMDisassembler.dir/ARMDisassembler.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMARMDisassembler.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[1902/3021] Linking CXX static library lib\libLLVMARMAsmParser.a
[1903/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFAbstractMemberAccess.cpp.obj
[1904/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFFrameLowering.cpp.obj
[1905/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFAsmPrinter.cpp.obj
[1905/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFAsmPrinter.cpp.obj
[1906/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFCheckAndAdjustIR.cpp.obj
[1907/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFInstrInfo.cpp.obj
[1908/3021] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFAdjustOpt.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 190.674 seconds
Build completed unsuccessfully in 0:04:48
Build completed unsuccessfully in 0:04:48
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
