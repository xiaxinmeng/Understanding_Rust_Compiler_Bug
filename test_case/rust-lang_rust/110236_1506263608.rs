plain
[2545/3021] Linking CXX static library lib\libLLVMTarget.a
[2546/3021] Linking CXX static library lib\libLLVMDWP.a
[2547/3021] Linking CXX static library lib\libLLVMScalarOpts.a
[2548/3021] Linking CXX static library lib\libLLVMExecutionEngine.a
FAILED: lib/libLLVMExecutionEngine.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMExecutionEngine.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMExecutionEngine.a  lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/ExecutionEngine.cpp.obj lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/ExecutionEngineBindings.cpp.obj lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/GDBRegistrationListener.cpp.obj lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/SectionMemoryManager.cpp.obj lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/TargetSelect.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMExecutionEngine.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2549/3021] Linking CXX static library lib\libLLVMFuzzMutate.a
[2550/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.obj
[2551/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/LogBuilderConsumer.cpp.obj
[2552/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2552/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2553/3021] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
[2554/3021] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilderPipelines.cpp.obj
[2555/3021] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 1905.789 seconds
Build completed unsuccessfully in 0:34:03
Build completed unsuccessfully in 0:34:03
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
