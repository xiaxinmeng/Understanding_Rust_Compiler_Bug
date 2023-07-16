plain
[2524/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordInitializer.cpp.obj
[2525/3021] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
[2526/3021] Linking CXX static library lib\libLLVMDlltoolDriver.a
FAILED: lib/libLLVMDlltoolDriver.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMDlltoolDriver.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMDlltoolDriver.a  lib/ToolDrivers/llvm-dlltool/CMakeFiles/LLVMDlltoolDriver.dir/DlltoolDriver.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMDlltoolDriver.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2527/3021] Linking CXX static library lib\libLLVMLibDriver.a
[2528/3021] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2529/3021] Linking CXX static library lib\libLLVMObjCopy.a
[2530/3021] Linking CXX static library lib\libLLVMJITLink.a
[2530/3021] Linking CXX static library lib\libLLVMJITLink.a
[2531/3021] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2532/3021] Linking CXX static library lib\libLLVMObjectYAML.a
[2533/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\cmake-0.1.48\src\lib.rs:975:5
 finished in 277.711 seconds
Build completed unsuccessfully in 0:06:21
Build completed unsuccessfully in 0:06:21
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
