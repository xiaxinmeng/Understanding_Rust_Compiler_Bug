plain
[2529/3025] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
[2530/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/BlockVerifier.cpp.obj
[2531/3025] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2532/3025] Linking CXX static library lib\libLLVMRuntimeDyld.a
FAILED: lib/libLLVMRuntimeDyld.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMRuntimeDyld.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMRuntimeDyld.a  lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/JITSymbol.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RTDyldMemoryManager.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyld.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldChecker.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldCOFF.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldELF.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldMachO.cpp.obj lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/Targets/RuntimeDyldELFMips.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMRuntimeDyld.a && cd ."
C:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2533/3025] Linking CXX static library lib\libLLVMObjCopy.a
[2534/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2535/3025] Linking CXX static library lib\libLLVMJITLink.a
[2536/3025] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2536/3025] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2537/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.obj
[2538/3025] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2539/3025] Linking CXX static library lib\libLLVMObjectYAML.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 280.692 seconds
Build completed unsuccessfully in 0:05:51
Build completed unsuccessfully in 0:05:51
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
