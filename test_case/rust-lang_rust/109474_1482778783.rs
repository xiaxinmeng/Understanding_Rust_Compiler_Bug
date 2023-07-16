plain
[2523/3103] Linking CXX static library lib\libLLVMMSP430Disassembler.a
[2524/3103] Linking CXX static library lib\libLLVMMSP430AsmParser.a
[2525/3103] Linking CXX static library lib\libLLVMMipsDisassembler.a
[2526/3103] Linking CXX static library lib\libLLVMNVPTXInfo.a
FAILED: lib/libLLVMNVPTXInfo.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMNVPTXInfo.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMNVPTXInfo.a  lib/Target/NVPTX/TargetInfo/CMakeFiles/LLVMNVPTXInfo.dir/NVPTXTargetInfo.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMNVPTXInfo.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2527/3103] Linking CXX static library lib\libLLVMPowerPCInfo.a
[2528/3103] Linking CXX static library lib\libLLVMMipsDesc.a
[2529/3103] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/TextStubCommon.cpp.obj
[2530/3103] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/InterfaceFile.cpp.obj
[2530/3103] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/InterfaceFile.cpp.obj
[2531/3103] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/StandardInstrumentations.cpp.obj
[2532/3103] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilderPipelines.cpp.obj
[2533/3103] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 2142.188 seconds
Build completed unsuccessfully in 0:36:55
Build completed unsuccessfully in 0:36:55
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
