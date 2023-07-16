
[1432/1629] Building CXX object lib/Target/MSP430/InstPrinter/CMakeFiles/LLVMMSP430AsmPrinter.dir/MSP430InstPrinter.cpp.obj
DEBUG:sccache::commands: Server sent CompileStarted
[1433/1629] Linking CXX static library lib\libLLVMMSP430AsmPrinter.a
FAILED: lib/libLLVMMSP430AsmPrinter.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMMSP430AsmPrinter.a && C:\projects\rust\mingw32\bin\ar.exe qc lib\libLLVMMSP430AsmPrinter.a  lib/Target/MSP430/InstPrinter/CMakeFiles/LLVMMSP430AsmPrinter.dir/MSP430InstPrinter.cpp.obj && C:\projects\rust\mingw32\bin\ranlib.exe lib\libLLVMMSP430AsmPrinter.a && cd ."
C:\projects\rust\mingw32\bin\ar.exe: unable to rename 'lib\libLLVMMSP430AsmPrinter.a'; reason: Permission denied
[1434/1629] Linking CXX static library lib\libLLVMMSP430CodeGen.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 380.247
Build completed unsuccessfully in 0:08:32
Command exited with code 101
