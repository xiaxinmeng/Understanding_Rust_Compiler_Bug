plain
[2625/3025] Linking CXX static library lib\libLLVMDWARFLinker.a
[2626/3025] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/disassemble.c.obj
[2627/3025] Linking CXX static library lib\libLLVMWebAssemblyDisassembler.a
FAILED: lib/libLLVMWebAssemblyDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMWebAssemblyDisassembler.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMWebAssemblyDisassembler.a  lib/Target/WebAssembly/Disassembler/CMakeFiles/LLVMWebAssemblyDisassembler.dir/WebAssemblyDisassembler.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMWebAssemblyDisassembler.a && cd ."
D:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2628/3025] Linking CXX static library lib\libLLVMSelectionDAG.a
[2629/3025] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/object.c.obj
[2630/3025] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/helpers.c.obj
[2631/3025] Linking CXX executable bin\llvm-bcanalyzer.exe
[2631/3025] Linking CXX executable bin\llvm-bcanalyzer.exe
[2632/3025] Linking CXX executable bin\llvm-profdata.exe
[2633/3025] Linking CXX executable bin\llvm-as.exe
[2634/3025] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 229.322 seconds
Build completed successfully in 0:05:05
Build completed successfully in 0:05:05
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
