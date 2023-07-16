plain
[466/3025] Linking CXX static library lib\libLLVMLineEditor.a
[467/3025] Linking CXX static library lib\libLLVMTableGenGlobalISel.a
[468/3025] Linking CXX static library lib\libLLVMWindowsDriver.a
FAILED: lib/libLLVMWindowsDriver.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMWindowsDriver.a && C:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMWindowsDriver.a  lib/WindowsDriver/CMakeFiles/LLVMWindowsDriver.dir/MSVCPaths.cpp.obj && C:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMWindowsDriver.a && cd ."
C:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[469/3025] Linking CXX static library lib\libLLVMTextAPI.a
[470/3025] Linking CXX static library lib\libLLVMDebugInfoCodeView.a
[471/3025] Linking CXX executable bin\FileCheck.exe
[471/3025] Linking CXX executable bin\FileCheck.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 56.046 seconds
Build completed unsuccessfully in 0:03:11
Build completed unsuccessfully in 0:03:11
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
