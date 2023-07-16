plain
[2532/3025] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2533/3025] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2534/3025] Linking CXX static library lib\libLLVMLibDriver.a
FAILED: lib/libLLVMLibDriver.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMLibDriver.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMLibDriver.a  lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMLibDriver.a && cd ."
C:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2535/3025] Linking CXX static library lib\libLLVMJITLink.a
[2536/3025] Linking CXX static library lib\libLLVMObjectYAML.a
[2537/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/LogBuilderConsumer.cpp.obj
[2538/3025] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2538/3025] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2539/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2540/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.obj
[2541/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 280.221 seconds
Build completed unsuccessfully in 0:06:57
Build completed unsuccessfully in 0:06:57
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
