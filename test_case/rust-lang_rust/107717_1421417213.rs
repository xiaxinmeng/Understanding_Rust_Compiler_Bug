plain
[2571/3025] Linking CXX executable bin\FileCheck.exe
[2572/3025] Linking CXX static library lib\libLLVMExecutionEngine.a
[2573/3025] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2574/3025] Linking CXX static library lib\libLLVMMCJIT.a
FAILED: lib/libLLVMMCJIT.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMMCJIT.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMMCJIT.a  lib/ExecutionEngine/MCJIT/CMakeFiles/LLVMMCJIT.dir/MCJIT.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMMCJIT.a && cd ."
D:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2575/3025] Linking CXX static library lib\libLLVMTransformUtils.a
[2576/3025] Linking CXX executable bin\yaml-bench.exe
[2577/3025] Building CXX object tools/llvm-lto/CMakeFiles/llvm-lto.dir/llvm-lto.cpp.obj
[2578/3025] Linking CXX executable bin\llvm-config.exe
[2578/3025] Linking CXX executable bin\llvm-config.exe
[2579/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/BugDriver.cpp.obj
[2580/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExecutionDriver.cpp.obj
[2581/3025] Building CXX object tools/llvm-profdata/CMakeFiles/llvm-profdata.dir/llvm-profdata.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 267.167 seconds
Build completed successfully in 0:07:42
