plain
[2526/3021] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2527/3021] Linking CXX static library lib\libLLVMDlltoolDriver.a
[2528/3021] Linking CXX static library lib\libLLVMLibDriver.a
[2529/3021] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
FAILED: lib/libLLVMDebugInfoGSYM.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMDebugInfoGSYM.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMDebugInfoGSYM.a  lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/DwarfTransformer.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/Header.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LookupResult.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ObjectFileTransformer.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ExtractRanges.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMDebugInfoGSYM.a && cd ."
D:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2530/3021] Linking CXX static library lib\libLLVMObjectYAML.a
[2531/3021] Linking CXX static library lib\libLLVMJITLink.a
[2532/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.obj
[2533/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2533/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2534/3021] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2535/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordInitializer.cpp.obj
[2536/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 230.694 seconds
Build completed unsuccessfully in 0:05:12
Build completed unsuccessfully in 0:05:12
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
