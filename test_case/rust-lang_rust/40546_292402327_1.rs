
[1070/1629] Linking CXX static library lib\libLLVMAArch64Disassembler.a
FAILED: lib/libLLVMAArch64Disassembler.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMAArch64Disassembler.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMAArch64Disassembler.a  lib/Target/AArch64/Disassembler/CMakeFiles/LLVMAArch64Disassembler.dir/AArch64Disassembler.cpp.obj lib/Target/AArch64/Disassembler/CMakeFiles/LLVMAArch64Disassembler.dir/AArch64ExternalSymbolizer.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMAArch64Disassembler.a && cd ."
C:\projects\rust\mingw64\bin\ar.exe: unable to rename 'lib\libLLVMAArch64Disassembler.a'; reason: No such file or directory
[1071/1629] Linking CXX static library lib\libLLVMAArch64AsmPrinter.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 264.200
Build completed unsuccessfully in 0:06:26
Command exited with code 101
