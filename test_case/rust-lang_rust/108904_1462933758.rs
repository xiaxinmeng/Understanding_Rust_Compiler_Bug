plain
[2616/3025] Linking CXX static library lib\libLLVMMIRParser.a
[2617/3025] Building CXX object tools/lli/CMakeFiles/lli.dir/ExecutionUtils.cpp.obj
[2618/3025] Linking CXX static library lib\libLLVMInterpreter.a
FAILED: lib/libLLVMInterpreter.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMInterpreter.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMInterpreter.a  lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Execution.cpp.obj lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/ExternalFunctions.cpp.obj lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Interpreter.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMInterpreter.a && cd ."
D:\a\rust\rust\mingw64\bin\ranlib.exe: could not create temporary file whilst writing archive: no more archived files
[2619/3025] Linking CXX static library lib\libLLVMDWARFLinker.a
[2620/3025] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2621/3025] Building CXX object tools/lli/ChildTarget/CMakeFiles/lli-child-target.dir/ChildTarget.cpp.obj
[2622/3025] Linking CXX static library lib\libLLVMSelectionDAG.a
[2622/3025] Linking CXX static library lib\libLLVMSelectionDAG.a
[2623/3025] Building CXX object tools/llvm-as/CMakeFiles/llvm-as.dir/llvm-as.cpp.obj
[2624/3025] Linking CXX executable bin\llvm-profdata.exe
[2625/3025] Linking CXX static library lib\libLLVMPasses.a
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 219.823 seconds
Build completed unsuccessfully in 0:04:52
Build completed unsuccessfully in 0:04:52
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
