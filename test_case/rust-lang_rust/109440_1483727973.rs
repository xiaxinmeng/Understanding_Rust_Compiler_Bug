plain
[2557/3021] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar-driver.cpp.obj
[2558/3021] Linking CXX static library lib\libLLVMAnalysis.a
[2559/3021] Building CXX object tools/lto/CMakeFiles/LTO.dir/LTODisassembler.cpp.obj
[2560/3021] Linking CXX static library lib\libLLVMTransformUtils.a
FAILED: lib/libLLVMTransformUtils.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMTransformUtils.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMTransformUtils.a  @CMakeFiles\LLVMTransformUtils.rsp && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMTransformUtils.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2561/3021] Linking CXX static library lib\libLLVMBitWriter.a
[2562/3021] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.obj
[2563/3021] Linking CXX executable bin\FileCheck.exe
[2564/3021] Linking CXX executable bin\llvm-PerfectShuffle.exe
[2564/3021] Linking CXX executable bin\llvm-PerfectShuffle.exe
[2565/3021] Building CXX object tools/llvm-config/CMakeFiles/llvm-config.dir/llvm-config.cpp.obj
[2566/3021] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2567/3021] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 303.246 seconds
Build completed unsuccessfully in 0:07:27
Build completed unsuccessfully in 0:07:27
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
