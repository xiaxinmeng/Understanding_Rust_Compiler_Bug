plain
      | 
D:/a/rust/rust/src/llvm-project/llvm/include/llvm/Support/FormatProviders.h:67: note: adding '-flarge-source-files' will allow for more column-tracking support, at the expense of compilation time and memory
[2757/3025] Building CXX object tools/llvm-exegesis/lib/Mips/CMakeFiles/LLVMExegesisMips.dir/Target.cpp.obj
[2758/3025] Linking CXX static library lib\libLLVMExegesisMips.a
FAILED: lib/libLLVMExegesisMips.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMExegesisMips.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMExegesisMips.a  tools/llvm-exegesis/lib/Mips/CMakeFiles/LLVMExegesisMips.dir/Target.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMExegesisMips.a && cd ."
D:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2759/3025] Linking CXX static library lib\libLLVMExegesisX86.a
[2760/3025] Linking CXX executable bin\llvm-dwarfdump.exe
[2761/3025] Linking CXX executable bin\llvm-extract.exe
[2762/3025] Linking CXX shared library bin\libLTO.dll
---

[2765/3025] Linking CXX executable bin\llvm-dwarfutil.exe
lib/libLLVMM68kCodeGen.a(M68kCallLowering.cpp.obj): duplicate section `.rdata$_ZTVN4llvm16FormalArgHandlerE[_ZTVN4llvm16FormalArgHandlerE]' has different size

ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 272.850 seconds
Build completed unsuccessfully in 0:05:53
Build completed unsuccessfully in 0:05:53
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
