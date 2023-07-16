
[1317/1629] Linking CXX static library lib\libLLVMSystemZDisassembler.a
FAILED: lib/libLLVMSystemZDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMSystemZDisassembler.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMSystemZDisassembler.a  lib/Target/SystemZ/Disassembler/CMakeFiles/LLVMSystemZDisassembler.dir/SystemZDisassembler.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMSystemZDisassembler.a && cd ."
C:\projects\rust\mingw64\bin\ar.exe: unable to rename 'lib\libLLVMSystemZDisassembler.a'; reason: No such file or directory
[1318/1629] Linking CXX static library lib\libLLVMSystemZAsmParser.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 357.723
Build completed unsuccessfully in 0:08:28
Command exited with code 101
