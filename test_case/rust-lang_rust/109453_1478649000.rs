plain
[2529/3025] Linking CXX static library lib\libLLVMObjCopy.a
[2530/3025] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2531/3025] Linking CXX static library lib\libLLVMLibDriver.a
[2532/3025] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
FAILED: lib/libLLVMDebugInfoGSYM.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMDebugInfoGSYM.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMDebugInfoGSYM.a  lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/DwarfTransformer.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/Header.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LookupResult.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ObjectFileTransformer.cpp.obj lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ExtractRanges.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMDebugInfoGSYM.a && cd ."
C:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2533/3025] Linking CXX static library lib\libLLVMJITLink.a
[2534/3025] Linking CXX static library lib\libLLVMDlltoolDriver.a
[2535/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.obj
[2536/3025] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2536/3025] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2537/3025] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.obj
[2538/3025] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2539/3025] Linking CXX static library lib\libLLVMObjectYAML.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 271.764 seconds
Build completed unsuccessfully in 0:06:41
Build completed unsuccessfully in 0:06:41
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
