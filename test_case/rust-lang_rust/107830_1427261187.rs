plain
[2526/3021] Linking CXX static library lib\libLLVMObjCopy.a
[2527/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.obj
[2528/3021] Linking CXX static library lib\libLLVMDebugInfoDWARF.a
[2529/3021] Linking CXX static library lib\libLLVMJITLink.a
FAILED: lib/libLLVMJITLink.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMJITLink.a && D:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMJITLink.a  lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/DWARFRecordSectionSplitter.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/EHFrameSupport.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLink.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkGeneric.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkMemoryManager.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MemoryFlags.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_arm64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachOLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELFLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_aarch64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_riscv.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFF.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFFLinkGraphBuilder.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/COFF_x86_64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/aarch64.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/riscv.cpp.obj lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/x86_64.cpp.obj && D:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMJITLink.a && cd ."
D:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2530/3021] Linking CXX static library lib\libLLVMDlltoolDriver.a
[2531/3021] Linking CXX static library lib\libLLVMLibDriver.a
[2532/3021] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.obj
[2533/3021] Linking CXX static library lib\libLLVMObjectYAML.a
[2533/3021] Linking CXX static library lib\libLLVMObjectYAML.a
[2534/3021] Linking CXX static library lib\libLLVMDebugInfoGSYM.a
[2535/3021] Linking CXX static library lib\libLLVMRuntimeDyld.a
[2536/3021] Linking CXX static library lib\libLLVMDebugInfoPDB.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 265.591 seconds
Build completed unsuccessfully in 0:07:33
Build completed unsuccessfully in 0:07:33
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
