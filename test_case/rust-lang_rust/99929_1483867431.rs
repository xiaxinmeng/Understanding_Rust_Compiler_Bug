plain
[2588/3021] Linking CXX static library lib\libLLVMCodeGen.a
[2589/3021] Linking CXX static library lib\libLLVMCoroutines.a
[2590/3021] Linking CXX static library lib\libLLVMAsmPrinter.a
FAILED: lib/libLLVMAsmPrinter.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMAsmPrinter.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMAsmPrinter.a  lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AccelTable.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AddressPool.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AIXException.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/ARMException.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AsmPrinter.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AsmPrinterDwarf.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AsmPrinterInlineAsm.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DbgEntityHistoryCalculator.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DebugHandlerBase.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DebugLocStream.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DIE.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DIEHash.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfCFIException.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfCompileUnit.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfDebug.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfExpression.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfFile.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfStringPool.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfUnit.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/EHStreamer.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/ErlangGCPrinter.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/OcamlGCPrinter.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/PseudoProbePrinter.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/WinCFGuard.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/WinException.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/CodeViewDebug.cpp.obj lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/WasmException.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMAsmPrinter.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2591/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExecutionDriver.cpp.obj
[2592/3021] Linking CXX static library lib\libLLVMSelectionDAG.a
[2593/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/OptimizerDriver.cpp.obj
[2594/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExtractFunction.cpp.obj
[2594/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExtractFunction.cpp.obj
[2595/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/CrashDebugger.cpp.obj
[2596/3021] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ToolRunner.cpp.obj
[2597/3021] Linking CXX executable bin\llvm-profdata.exe
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 300.488 seconds
Build completed unsuccessfully in 0:07:58
