plain
[2510/3103] Linking CXX static library lib\libLLVMBitReader.a
[2511/3103] Linking CXX static library lib\libLLVMFrontendHLSL.a
[2512/3103] Linking CXX static library lib\libLLVMCFGuard.a
[2513/3103] Linking CXX static library lib\libLLVMMCA.a
FAILED: lib/libLLVMMCA.a 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E rm -f lib\libLLVMMCA.a && C:\a\rust\rust\mingw32\bin\ar.exe qc lib\libLLVMMCA.a  lib/MCA/CMakeFiles/LLVMMCA.dir/CodeEmitter.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Context.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/CustomBehaviour.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HWEventListener.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/HardwareUnit.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/LSUnit.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RegisterFile.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/ResourceManager.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/Scheduler.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/IncrementalSourceMgr.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/DispatchStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/EntryStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/ExecuteStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/InOrderIssueStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/InstructionTables.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/MicroOpQueueStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/RetireStage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/Stage.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/Support.cpp.obj lib/MCA/CMakeFiles/LLVMMCA.dir/View.cpp.obj && C:\a\rust\rust\mingw32\bin\ranlib.exe lib\libLLVMMCA.a && cd ."
C:\a\rust\rust\mingw32\bin\ar.exe: could not create temporary file whilst writing archive: no more archived files
[2514/3103] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/TextStubCommon.cpp.obj
[2515/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.obj
[2516/3103] Linking CXX static library lib\libLLVMAArch64Info.a
[2517/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.obj
[2517/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.obj
[2518/3103] Building CXX object lib/ToolDrivers/llvm-dlltool/CMakeFiles/LLVMDlltoolDriver.dir/DlltoolDriver.cpp.obj
[2519/3103] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.obj
[2520/3103] Building CXX object lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 314.983 seconds
Build completed unsuccessfully in 0:08:02
