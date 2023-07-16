
[1224/1629] Linking CXX static library lib\libLLVMMipsDisassembler.a
FAILED: lib/libLLVMMipsDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMMipsDisassembler.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMMipsDisassembler.a  lib/Target/Mips/Disassembler/CMakeFiles/LLVMMipsDisassembler.dir/MipsDisassembler.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMMipsDisassembler.a && cd ."
C:\projects\rust\mingw32\bin\ar.exe: unable to rename 'lib\libLLVMMipsDisassembler.a'; reason: No such file or directory
[1225/1629] Linking CXX static library lib\libLLVMMipsAsmPrinter.a
FAILED: lib/libLLVMMipsAsmPrinter.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMMipsAsmPrinter.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMMipsAsmPrinter.a  lib/Target/Mips/InstPrinter/CMakeFiles/LLVMMipsAsmPrinter.dir/MipsInstPrinter.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMMipsAsmPrinter.a && cd ."
C:\projects\rust\mingw32\bin\ranlib.exe: lib\libLLVMMipsAsmPrinter.a: Malformed archive
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 320.843
Build completed unsuccessfully in 0:07:19
Command exited with code 101
