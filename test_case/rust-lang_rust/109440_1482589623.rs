text
[2958/3021] Linking CXX static library lib\libLLVMExegesisAArch64.a
FAILED: lib/libLLVMExegesisAArch64.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMExegesisAArch64.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMExegesisAArch64.a  tools/llvm-exegesis/lib/AArch64/CMakeFiles/LLVMExegesisAArch64.dir/Target.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMExegesisAArch64.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files

[2959/3021] Linking CXX static library lib\libLLVMExegesisX86.a
[2960/3021] Linking CXX executable bin\llvm-dwp.exe
lib/libLLVMM68kCodeGen.a(M68kCallLowering.cpp.obj): duplicate section `.rdata$_ZTVN4llvm16FormalArgHandlerE[__ZTVN4llvm16FormalArgHandlerE]' has different size

[2961/3021] Linking CXX executable bin\llvm-dwarfutil.exe
lib/libLLVMM68kCodeGen.a(M68kCallLowering.cpp.obj): duplicate section `.rdata$_ZTVN4llvm16FormalArgHandlerE[__ZTVN4llvm16FormalArgHandlerE]' has different size

ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
