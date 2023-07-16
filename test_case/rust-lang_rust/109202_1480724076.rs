plain
[2528/3021] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2529/3021] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2530/3021] Linking CXX static library lib\libLLVMLibDriver.a
FAILED: lib/libLLVMLibDriver.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMLibDriver.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMLibDriver.a  lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMLibDriver.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2531/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2532/3021] Linking CXX static library lib\libLLVMDlltoolDriver.a
[2533/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.obj
[2534/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.obj
[2534/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.obj
[2535/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.obj
[2536/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.obj
[2537/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 301.255 seconds
Build completed unsuccessfully in 0:07:24
Build completed unsuccessfully in 0:07:24
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
