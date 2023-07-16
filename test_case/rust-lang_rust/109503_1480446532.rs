plain
[2523/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.obj
[2524/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Trace.cpp.obj
[2525/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.obj
[2526/3021] Linking CXX static library lib\libLLVMJITLink.a
FAILED: lib/libLLVMJITLink.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMJITLink.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMJITLink.a  lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/DWARFRecordSectionSplitter.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/EHFrameSupport.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLink.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkGeneric.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkMemoryManager.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MemoryFlags.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_arm64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachOLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELFLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_aarch64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_riscv.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFF.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFFLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFF_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/aarch64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/riscv.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/x86_64.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMJITLink.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2527/3021] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2528/3021] Linking CXX static library lib\libLLVMObjectYAML.a
[2529/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.obj
[2530/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2530/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
[2531/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.obj
[2532/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2533/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 295.896 seconds
Build completed unsuccessfully in 0:07:18
Build completed unsuccessfully in 0:07:18
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
