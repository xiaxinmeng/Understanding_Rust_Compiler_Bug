
[1022/1629] Linking CXX static library lib\libLLVMX86AsmPrinter.a
FAILED: lib/libLLVMX86AsmPrinter.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMX86AsmPrinter.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMX86AsmPrinter.a  lib/Target/X86/InstPrinter/CMakeFiles/LLVMX86AsmPrinter.dir/X86ATTInstPrinter.cpp.obj lib/Target/X86/InstPrinter/CMakeFiles/LLVMX86AsmPrinter.dir/X86IntelInstPrinter.cpp.obj lib/Target/X86/InstPrinter/CMakeFiles/LLVMX86AsmPrinter.dir/X86InstComments.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMX86AsmPrinter.a && cd ."
C:\projects\rust\mingw32\bin\ar.exe: unable to rename 'lib\libLLVMX86AsmPrinter.a'; reason: Permission denied
[1023/1629] Linking CXX static library lib\libLLVMX86Disassembler.a
FAILED: lib/libLLVMX86Disassembler.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMX86Disassembler.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMX86Disassembler.a  lib/Target/X86/Disassembler/CMakeFiles/LLVMX86Disassembler.dir/X86Disassembler.cpp.obj lib/Target/X86/Disassembler/CMakeFiles/LLVMX86Disassembler.dir/X86DisassemblerDecoder.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMX86Disassembler.a && cd ."
C:\projects\rust\mingw32\bin\ranlib.exe: lib\libLLVMX86Disassembler.a: Malformed archive
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

