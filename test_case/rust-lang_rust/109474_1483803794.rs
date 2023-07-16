plain
[2604/3103] Linking CXX static library lib\libLLVMDebugInfoLogicalView.a
[2605/3103] Linking CXX static library lib\libLLVMSymbolize.a
[2606/3103] Linking CXX static library lib\libLLVMWindowsDriver.a
FAILED: lib/libLLVMWindowsDriver.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMWindowsDriver.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMWindowsDriver.a  lib/WindowsDriver/CMakeFiles/LLVMWindowsDriver.dir/MSVCPaths.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMWindowsDriver.a && cd ."
C:\a\rust\rust\mingw32\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2607/3103] Linking CXX static library lib\libLLVMWindowsManifest.a
[2608/3103] Linking CXX static library lib\libLLVMXRay.a
[2609/3103] Building CXX object utils/UnicodeData/CMakeFiles/UnicodeNameMappingGenerator.dir/UnicodeNameMappingGenerator.cpp.obj
[2610/3103] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.obj
[2610/3103] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.obj
[2611/3103] Building CXX object utils/split-file/CMakeFiles/split-file.dir/split-file.cpp.obj
[2612/3103] Linking CXX executable bin\FileCheck.exe
[2613/3103] Building CXX object utils/not/CMakeFiles/not.dir/not.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 342.444 seconds
Build completed unsuccessfully in 0:08:07
Build completed unsuccessfully in 0:08:07
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
