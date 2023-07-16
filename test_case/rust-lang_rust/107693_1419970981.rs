plain
[2568/3021] Linking CXX static library lib\libLLVMAggressiveInstCombine.a
[2569/3021] Linking CXX static library lib\libLLVMInstrumentation.a
[2570/3021] Linking CXX static library lib\libLLVMLinker.a
FAILED: lib/libLLVMLinker.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMLinker.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMLinker.a  lib/Linker/CMakeFiles/LLVMLinker.dir/IRMover.cpp.obj lib/Linker/CMakeFiles/LLVMLinker.dir/LinkModules.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMLinker.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2571/3021] Linking CXX static library lib\libLLVMObjCARCOpts.a
[2572/3021] Linking CXX static library lib\libLLVMInstCombine.a
[2573/3021] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2574/3021] Linking CXX static library lib\libLLVMVectorize.a
[2574/3021] Linking CXX static library lib\libLLVMVectorize.a
[2575/3021] Linking CXX executable bin\yaml-bench.exe
[2576/3021] Linking CXX executable bin\UnicodeNameMappingGenerator.exe
[2577/3021] Linking CXX executable bin\not.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 381.544 seconds
Build completed successfully in 0:08:27
Build completed successfully in 0:08:27
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
