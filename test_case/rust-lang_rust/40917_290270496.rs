
FAILED: lib/libLLVMAArch64Info.a 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E remove lib\libLLVMAArch64Info.a && C:\projects\rust\mingw64\bin\ar.exe qc lib\libLLVMAArch64Info.a  lib/Target/AArch64/TargetInfo/CMakeFiles/LLVMAArch64Info.dir/AArch64TargetInfo.cpp.obj && C:\projects\rust\mingw64\bin\ranlib.exe lib\libLLVMAArch64Info.a && cd ."
C:\projects\rust\mingw64\bin\ranlib.exe: unable to rename 'lib\libLLVMAArch64Info.a'; reason: Permission denied
[1140/1629] Linking CXX static library lib\libLLVMAArch64CodeGen.a
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.22\src\lib.rs:617
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 271.254
Build completed unsuccessfully in 0:06:40
Command exited with code 101
cat %CD%/sccache.log
DEBUG:sccache::cache::cache: Trying S3Cache(rust-lang-ci-sccache.s3.amazonaws.com)
DEBUG:sccache::server: handle_client: compile
