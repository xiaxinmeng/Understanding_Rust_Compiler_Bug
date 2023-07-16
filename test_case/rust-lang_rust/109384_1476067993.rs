plain
[2526/3021] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2527/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.obj
[2528/3021] Linking CXX static library lib\libLLVMJITLink.a
[2529/3021] Linking CXX static library lib\libLLVMRuntimeDyld.a
FAILED: lib/libLLVMRuntimeDyld.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMRuntimeDyld.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMRuntimeDyld.a  lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/JITSymbol.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RTDyldMemoryManager.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyld.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldChecker.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldCOFF.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldELF.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldMachO.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/Targets/RuntimeDyldELFMips.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMRuntimeDyld.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2530/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2531/3021] Linking CXX static library lib\libLLVMDlltoolDriver.a
[2532/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.obj
[2533/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.obj
[2533/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.obj
[2534/3021] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
[2535/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.obj
[2536/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 278.920 seconds
Build completed unsuccessfully in 0:07:24
