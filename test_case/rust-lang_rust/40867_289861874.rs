
[1325/1629] Linking CXX static library lib\libLLVMSystemZDisassembler.a
FAILED: lib/libLLVMSystemZDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMSystemZDisassembler.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMSystemZDisassembler.a  lib/Target/SystemZ/Disassembler/CMakeFiles/LLVMSystemZDisassembler.dir/SystemZDisassembler.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMSystemZDisassembler.a && cd ."
C:\projects\rust\mingw64\bin\ar.exe: unable to rename 'lib\libLLVMSystemZDisassembler.a'; reason: No such file or directory
[1326/1629] Linking CXX static library lib\libLLVMSystemZAsmPrinter.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
