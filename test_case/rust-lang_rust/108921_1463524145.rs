plain
[2568/3021] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.obj
[2569/3021] Linking CXX static library lib\libLLVMAggressiveInstCombine.a
[2570/3021] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2571/3021] Linking CXX static library lib\libLLVMVectorize.a
FAILED: lib/libLLVMVectorize.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMVectorize.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMVectorize.a  lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoadStoreVectorizer.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoopVectorizationLegality.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoopVectorize.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/SLPVectorizer.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/Vectorize.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VectorCombine.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlan.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanHCFGBuilder.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanRecipes.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanSLP.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanTransforms.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanVerifier.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMVectorize.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2572/3021] Linking CXX static library lib\libLLVMInstrumentation.a
[2573/3021] Linking CXX static library lib\libLLVMObjCARCOpts.a
[2574/3021] Linking CXX executable bin\yaml-bench.exe
[2575/3021] Linking CXX static library lib\libLLVMInstCombine.a
[2575/3021] Linking CXX static library lib\libLLVMInstCombine.a
[2576/3021] Linking CXX executable bin\UnicodeNameMappingGenerator.exe
[2577/3021] Linking CXX executable bin\not.exe
[2578/3021] Linking CXX executable bin\llvm-config.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 307.979 seconds
Build completed unsuccessfully in 0:07:05
Build completed unsuccessfully in 0:07:05
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
