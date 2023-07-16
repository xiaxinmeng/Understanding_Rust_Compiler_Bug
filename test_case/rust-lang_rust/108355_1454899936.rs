plain
[2608/3025] Linking CXX static library lib\libLLVMAsmPrinter.a
[2609/3025] Linking CXX static library lib\libLLVMWebAssemblyUtils.a
[2610/3025] Linking CXX static library lib\libLLVMPasses.a
FAILED: lib/libLLVMPasses.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMPasses.a && D:\a\rust\rust\mingw64\bin\ar.exe qc lib\libLLVMPasses.a  lib/Passes/CMakeFiles/LLVMPasses.dir/OptimizationLevel.cpp.obj lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.obj lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilderBindings.cpp.obj lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilderPipelines.cpp.obj lib/Passes/CMakeFiles/LLVMPasses.dir/PassPlugin.cpp.obj lib/Passes/CMakeFiles/LLVMPasses.dir/StandardInstrumentations.cpp.obj && D:\a\rust\rust\mingw64\bin\ranlib.exe lib\libLLVMPasses.a && cd ."
D:\a\rust\rust\mingw64\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2611/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DwarfLinkerForBinary.cpp.obj
[2612/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachODebugMapParser.cpp.obj
[2613/3025] Linking CXX static library lib\libLLVMDWARFLinker.a
[2614/3025] Linking CXX static library lib\libLLVMWebAssemblyAsmParser.a
[2614/3025] Linking CXX static library lib\libLLVMWebAssemblyAsmParser.a
[2615/3025] Linking CXX static library lib\libLLVMSelectionDAG.a
[2616/3025] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil.cpp.obj
[2617/3025] Linking CXX executable bin\llvm-profdata.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cmake-0.1.48\src\lib.rs:975:5
 finished in 249.013 seconds
Build completed unsuccessfully in 0:07:12
