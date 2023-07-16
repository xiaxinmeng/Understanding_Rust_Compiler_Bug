
[1119/1629] Linking CXX static library lib\libLLVMAArch64Utils.a
FAILED: lib/libLLVMAArch64Utils.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMAArch64Utils.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMAArch64Utils.a  lib/Target/AArch64/Utils/CMakeFiles/LLVMAArch64Utils.dir/AArch64BaseInfo.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMAArch64Utils.a && cd ."
C:\projects\rust\mingw64\bin\ranlib.exe: unable to rename 'lib\libLLVMAArch64Utils.a'; reason: Permission denied
[1120/1629] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMAsmPrinter.cpp.obj
DEBUG:sccache::commands: Server sent CompileStarted
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 302.411
Build completed unsuccessfully in 0:07:34
Command exited with code 101
