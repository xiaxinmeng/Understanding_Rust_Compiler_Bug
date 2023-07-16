plain
[2577/3025] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar-driver.cpp.obj
[2578/3025] Linking CXX static library lib\libLLVMInstrumentation.a
[2579/3025] Linking CXX static library lib\libLLVMInstCombine.a
[2580/3025] Linking CXX static library lib\libLLVMVectorize.a
FAILED: lib/libLLVMVectorize.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMVectorize.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMVectorize.a  lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoadStoreVectorizer.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoopVectorizationLegality.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/LoopVectorize.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/SLPVectorizer.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/Vectorize.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VectorCombine.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlan.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanHCFGBuilder.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanRecipes.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanSLP.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanTransforms.cpp.obj lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanVerifier.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMVectorize.a && cd ."
C:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2581/3025] Linking CXX static library lib\libLLVMObjCARCOpts.a
[2582/3025] Linking CXX static library lib\libLLVMLinker.a
[2583/3025] Building CXX object tools/llvm-config/CMakeFiles/llvm-config.dir/llvm-config.cpp.obj
[2584/3025] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2584/3025] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.obj
[2585/3025] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.obj
[2586/3025] Building CXX object tools/llvm-lto/CMakeFiles/llvm-lto.dir/llvm-lto.cpp.obj
[2587/3025] Linking CXX executable bin\UnicodeNameMappingGenerator.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 279.889 seconds
Build completed unsuccessfully in 0:07:29
