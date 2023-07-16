plain
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: forksystems.mm.fcix.net
 * extras: forksystems.mm.fcix.net
 * updates: bay.uchicago.edu
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * extras: forksystems.mm.fcix.net
 * updates: bay.uchicago.edu
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * epel: dl.fedoraproject.org
 * extras: forksystems.mm.fcix.net
 * updates: bay.uchicago.edu
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
Resolving Dependencies
---
[ 92%] Built target LLVMDebuginfod
Scanning dependencies of target llvm-ifs
[ 92%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/ErrorCollector.cpp.o
[ 92%] Linking CXX shared library ../../lib/libLTO.so
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMCreateDisasm' failed: symbol not defined
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMCreateDisasmCPU' failed: symbol not defined
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMDisasmDispose' failed: symbol not defined
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMDisasmInstruction' failed: symbol not defined
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMSetDisasmOptions' failed: symbol not defined
ld.lld: error: version script assignment of 'LLVM_15' to symbol 'LLVMCreateDisasmCPUFeatures' failed: symbol not defined
[ 92%] Building CXX object tools/llvm-diff/lib/CMakeFiles/LLVMDiff.dir/DiffLog.cpp.o
[ 92%] Building CXX object tools/llvm-diff/lib/CMakeFiles/LLVMDiff.dir/DiffLog.cpp.o
clang-16: error: linker command failed with exit code 1 (use -v to see invocation)
gmake[2]: *** [lib/libLTO.so.15-rust-1.70.0-nightly] Error 1
gmake[1]: *** [tools/lto/CMakeFiles/LTO.dir/all] Error 2
gmake[1]: *** Waiting for unfinished jobs....
[ 92%] Linking CXX executable ../../bin/llvm-dis
[ 92%] Linking CXX static library ../../../lib/libLLVMCFIVerify.a
[ 92%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 92%] Built target LLVMCFIVerify
---
thread 'main' panicked at '
 finished in 993.065 seconds
command did not execute successfully, got: exit status: 2

build script failed, must exit now', /cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
Build completed unsuccessfully in 0:17:27
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (1047.95s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (1047.95s)
stage-build ERROR: The multi-stage build has failed
---
Total duration:                          17m 28s
------------------------------------------------
root INFO: Free disk space: 598.49 GiB out of total 666.61 GiB (10.22% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
