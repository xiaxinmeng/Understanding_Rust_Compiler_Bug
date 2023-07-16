
FAILED: lib/libLLVMMipsDesc.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMMipsDesc.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMMipsDesc.a  lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsABIInfo.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsABIFlagsSection.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsAsmBackend.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsELFObjectWriter.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsELFStreamer.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsMCAsmInfo.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsMCCodeEmitter.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsMCExpr.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsMCTargetDesc.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsNaClELFStreamer.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsOptionRecord.cpp.obj lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsTargetStreamer.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMMipsDesc.a && cd ."
C:\projects\rust\mingw64\bin\ranlib.exe: lib\libLLVMMipsDesc.a: Permission denied
[1274/1629] Linking CXX static library lib\libLLVMMipsAsmParser.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 308.299
Build completed unsuccessfully in 0:07:12
Command exited with code 101
