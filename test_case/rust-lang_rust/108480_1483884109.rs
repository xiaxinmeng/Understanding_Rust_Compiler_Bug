plain
[1298/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDebugAbbrev.cpp.obj
[1299/3025] Linking CXX static library lib\libLLVMCore.a
[1300/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDebugAddr.cpp.obj
[1301/3025] Linking CXX static library lib\libLLVMCFGuard.a
FAILED: lib/libLLVMCFGuard.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMCFGuard.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMCFGuard.a  lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMCFGuard.a && cd ."
C:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[1302/3025] Linking CXX static library lib\libLLVMBitReader.a
[1303/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFContext.cpp.obj
[1304/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDebugArangeSet.cpp.obj
[1305/3025] Building AArch64GenGlobalISel.inc...
[1305/3025] Building AArch64GenGlobalISel.inc...
[1306/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAcceleratorTable.cpp.obj
[1307/3025] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAbbreviationDeclaration.cpp.obj
[1308/3025] Building AArch64GenInstrInfo.inc...
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 119.950 seconds
Build completed unsuccessfully in 0:04:12
Build completed unsuccessfully in 0:04:12
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
