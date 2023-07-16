plain
[2555/3025] Linking CXX static library lib\libLLVMXRay.a
[2556/3025] Building CXX object utils/FileCheck/CMakeFiles/FileCheck.dir/FileCheck.cpp.obj
[2557/3025] Linking CXX static library lib\libLLVMWindowsManifest.a
[2558/3025] Linking CXX static library lib\libLLVMCoverage.a
FAILED: lib/libLLVMCoverage.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMCoverage.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMCoverage.a  lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMapping.cpp.obj lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingWriter.cpp.obj lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingReader.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMCoverage.a && cd ."
D:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2559/3025] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar-driver.cpp.obj
[2560/3025] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.obj
[2561/3025] Building CXX object lib/WindowsDriver/CMakeFiles/LLVMWindowsDriver.dir/MSVCPaths.cpp.obj
[2562/3025] Linking CXX executable bin\llvm-PerfectShuffle.exe
[2562/3025] Linking CXX executable bin\llvm-PerfectShuffle.exe
[2563/3025] Linking CXX executable bin\FileCheck.exe
[2564/3025] Linking CXX static library lib\libLLVMAnalysis.a
[2565/3025] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 254.121 seconds
Build completed unsuccessfully in 0:07:13
Build completed unsuccessfully in 0:07:13
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
