plain
[2576/3025] Linking CXX static library lib\libLLVMAggressiveInstCombine.a
[2577/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ToolRunner.cpp.obj
[2578/3025] Linking CXX static library lib\libLLVMInstrumentation.a
[2579/3025] Linking CXX static library lib\libLLVMInstCombine.a
FAILED: lib/libLLVMInstCombine.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMInstCombine.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMInstCombine.a  lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstructionCombining.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAddSub.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAndOrXor.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCompares.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineLoadStoreAlloca.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineMulDivRem.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineNegator.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombinePHI.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineSelect.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineShifts.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineSimplifyDemanded.cpp.obj lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineVectorOps.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMInstCombine.a && cd ."
C:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2580/3025] Linking CXX executable bin\UnicodeNameMappingGenerator.exe
[2581/3025] Linking CXX executable bin\llvm-config.exe
[2582/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExecutionDriver.cpp.obj
[2583/3025] Building CXX object tools/llvm-profdata/CMakeFiles/llvm-profdata.dir/llvm-profdata.cpp.obj
[2583/3025] Building CXX object tools/llvm-profdata/CMakeFiles/llvm-profdata.dir/llvm-profdata.cpp.obj
[2584/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/FindBugs.cpp.obj
[2585/3025] Linking CXX static library lib\libLLVMVectorize.a
[2586/3025] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/OptimizerDriver.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 291.332 seconds
Build completed unsuccessfully in 0:06:07
Build completed unsuccessfully in 0:06:07
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
