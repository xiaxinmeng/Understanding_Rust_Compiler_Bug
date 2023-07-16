
[1353/1629] Linking CXX static library lib\libLLVMJSBackendInfo.a
FAILED: lib/libLLVMJSBackendInfo.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMJSBackendInfo.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMJSBackendInfo.a  lib/Target/JSBackend/TargetInfo/CMakeFiles/LLVMJSBackendInfo.dir/JSBackendTargetInfo.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMJSBackendInfo.a && cd ."
C:\projects\rust\mingw64\bin\ar.exe: unable to rename 'lib\libLLVMJSBackendInfo.a'; reason: No such file or directory
[1354/1629] Linking CXX static library lib\libLLVMCoverage.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 332.638
Build completed unsuccessfully in 0:07:56
Command exited with code 101
