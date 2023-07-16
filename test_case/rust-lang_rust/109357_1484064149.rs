plain
[2525/3103] Linking CXX static library lib\libLLVMARMDisassembler.a
[2526/3103] Linking CXX static library lib\libLLVMARMAsmParser.a
[2527/3103] Linking CXX static library lib\libLLVMHexagonAsmParser.a
FAILED: lib/libLLVMHexagonAsmParser.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMHexagonAsmParser.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMHexagonAsmParser.a  lib/Target/Hexagon/AsmParser/CMakeFiles/LLVMHexagonAsmParser.dir/HexagonAsmParser.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMHexagonAsmParser.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2528/3103] Linking CXX static library lib\libLLVMMSP430Desc.a
[2529/3103] Building CXX object lib/ToolDrivers/llvm-dlltool/CMakeFiles/LLVMDlltoolDriver.dir/DlltoolDriver.cpp.obj
[2530/3103] Linking CXX static library lib\libLLVMAArch64AsmParser.a
[2531/3103] Linking CXX static library lib\libLLVMAArch64Disassembler.a
[2531/3103] Linking CXX static library lib\libLLVMAArch64Disassembler.a
[2532/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/BlockVerifier.cpp.obj
[2533/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecordProducer.cpp.obj
[2534/3103] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 321.489 seconds
Build completed unsuccessfully in 0:06:32
Build completed unsuccessfully in 0:06:32
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
