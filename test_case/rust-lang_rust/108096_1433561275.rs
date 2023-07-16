plain
[2629/3021] Linking CXX static library lib\libLLVMWebAssemblyDesc.a
[2630/3021] Linking CXX static library lib\libLLVMM68kAsmParser.a
[2631/3021] Linking CXX static library lib\libLLVMWebAssemblyDisassembler.a
FAILED: lib/libLLVMWebAssemblyDisassembler.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMWebAssemblyDisassembler.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMWebAssemblyDisassembler.a  lib/Target/WebAssembly/Disassembler/CMakeFiles/LLVMWebAssemblyDisassembler.dir/WebAssemblyDisassembler.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMWebAssemblyDisassembler.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2632/3021] Linking CXX static library lib\libLLVMWebAssemblyCodeGen.a
[2633/3021] Linking CXX static library lib\libLLVMX86CodeGen.a
[2634/3021] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/SymbolMap.cpp.obj
[2635/3021] Building CXX object tools/lli/CMakeFiles/lli.dir/ExecutionUtils.cpp.obj
[2635/3021] Building CXX object tools/lli/CMakeFiles/lli.dir/ExecutionUtils.cpp.obj
[2636/3021] Linking CXX executable bin\llvm-profdata.exe
[2637/3021] Building CXX object tools/lli/CMakeFiles/lli.dir/lli.cpp.obj
[2638/3021] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 268.496 seconds
Build completed unsuccessfully in 0:07:32
Build completed unsuccessfully in 0:07:32
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
