
In file included from C:\projects\rust\src\llvm\lib\Target\SystemZ\AsmParser\SystemZAsmParser.cpp:491:0:
lib/Target/SystemZ/SystemZGenAsmMatcher.inc:3156:6: warning: 'bool {anonymous}::SystemZAsmParser::mnemonicIsValid(llvm::StringRef, unsigned int)' defined but not used [-Wunused-function]
 bool SystemZAsmParser::
      ^~~~~~~~~~~~~~~~
[1242/1629] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZTDC.cpp.obj
DEBUG:sccache::commands: Server sent CompileStarted
[1243/1629] Linking CXX static library lib\libLLVMSystemZAsmParser.a
FAILED: lib/libLLVMSystemZAsmParser.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMSystemZAsmParser.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMSystemZAsmParser.a  lib/Target/SystemZ/AsmParser/CMakeFiles/LLVMSystemZAsmParser.dir/SystemZAsmParser.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMSystemZAsmParser.a && cd ."
C:\projects\rust\mingw32\bin\ranlib.exe: unable to rename 'lib\libLLVMSystemZAsmParser.a'; reason: Permission denied
[1244/1629] Linking CXX static library lib\libLLVMSystemZCodeGen.a
FAILED: lib/libLLVMSystemZCodeGen.a 
