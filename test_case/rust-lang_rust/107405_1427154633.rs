plain
[2603/3025] Linking CXX static library lib\libLLVMMIRParser.a
[2604/3025] Building CXX object tools/lli/CMakeFiles/lli.dir/lli.cpp.obj
[2605/3025] Linking CXX static library lib\libLLVMInterpreter.a
FAILED: lib/libLLVMInterpreter.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMInterpreter.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMInterpreter.a  lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Execution.cpp.obj lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/ExternalFunctions.cpp.obj lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Interpreter.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMInterpreter.a && cd ."
D:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2606/3025] Linking CXX static library lib\libLLVMCoroutines.a
[2607/3025] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2608/3025] Linking CXX static library lib\libLLVMAsmPrinter.a
[2609/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DebugMap.cpp.obj
[2609/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DebugMap.cpp.obj
[2610/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil.cpp.obj
[2611/3025] Linking CXX static library lib\libLLVMSelectionDAG.a
[2612/3025] Linking CXX executable bin\llvm-profdata.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 225.635 seconds
Build completed unsuccessfully in 0:06:21
