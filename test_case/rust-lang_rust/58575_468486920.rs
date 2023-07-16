plain
travis_time:end:0064d5b0:start=1551387260051075933,finish=1551387374574220534,duration=114523144601
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:30] Step 5/11 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:30]  ---> Running in 3b157bbe4579
[00:01:30] + TARGET=x86_64-linux-musl
[00:01:30] + ARCH=x86_64
[00:01:30] + OUTPUT=/usr/local
[00:01:30] + shift
[00:01:30] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:30] Cloning into 'musl-cross-make'...
[00:01:31] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:31] 
[00:01:31] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:31] changes and commit them, and you can discard any commits you make in this
[00:01:31] state without impacting any branches by performing another checkout.
[00:01:31] 
[00:01:31] If you want to create a new branch to retain commits you create, you may
[00:01:31] do so (now or later) by using -b with the checkout command again. Example:
[00:01:31] 
[00:01:31]   git checkout -b <new-branch-name>
[00:01:31] + cd musl-cross-make
[00:01:31] ++ nproc
[00:01:31] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:31] + set +x
---
[00:11:01] Thu Feb 28 21:07:24 UTC 2019 - building ...
[00:11:31] Thu Feb 28 21:07:54 UTC 2019 - building ...
[00:12:01] Thu Feb 28 21:08:24 UTC 2019 - building ...
[00:12:31] Thu Feb 28 21:08:54 UTC 2019 - building ...
[00:12:46] musl-toolchain.sh: line 3:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:46] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:13:02] musl-toolchain.sh: line 3:  6335 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:13:02] + cd -
[00:13:02] + cd -
[00:13:02] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:13:02] + echo /usr/local/x86_64-linux-musl/lib
[00:13:02] + export CC=x86_64-linux-musl-gcc
[00:13:02] + CC=x86_64-linux-musl-gcc
[00:13:02] + export CXX=x86_64-linux-musl-g++
---
[00:16:08] Step 8/11 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:16:08]  ---> Running in 16c06372933a
[00:16:08] Removing intermediate container 16c06372933a
[00:16:08]  ---> 72c880bb6050
[00:16:08] Step 9/11 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:16:08] Removing intermediate container 527a8ce1c260
[00:16:08]  ---> dd25291d995b
[00:16:08]  ---> dd25291d995b
[00:16:08] Step 10/11 : ENV RUSTFLAGS="-C target-feature=-crt-static"
[00:16:09] Removing intermediate container f3ccde22bd6b
[00:16:09]  ---> 4677b33fb6fa
[00:16:09] Step 11/11 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:16:09]  ---> Running in 2f651130caa5
[00:16:09]  ---> Running in 2f651130caa5
[00:16:09] Removing intermediate container 2f651130caa5
[00:16:09]  ---> d80873abf024
[00:16:09] Successfully built d80873abf024
[00:16:09] Successfully tagged rust-ci:latest
[00:16:09] Built container sha256:d80873abf02485ad5105db331547c4c0e7954494bdf6e1603eb891536d1bae8a
[00:16:09] Uploading finished image to s3://rust-lang-ci-sccache2/docker/fd3426e13b5752ae462db71e06d5b48945a4ba81170afb8377771e78b27939ac4e9cf4cf1a6079d885d0e41b8cb27e1d6b2c067dccae190830ff6799b32ece21
[00:22:35] upload failed: - to s3://rust-lang-ci-sccache2/docker/fd3426e13b5752ae462db71e06d5b48945a4ba81170afb8377771e78b27939ac4e9cf4cf1a6079d885d0e41b8cb27e1d6b2c067dccae190830ff6799b32ece21 Unable to locate credentials

[00:22:36] travis_time:end:1a517b4e:start=1551387408077792867,finish=1551388739196802559,duration=1331119009692
[CI_JOB_NAME=dist-x86_64-musl]
[00:22:36] [CI_JOB_NAME=dist-x86_64-musl]
---

[01:12:28] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[01:12:28] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-linux-musl-gcc" "-DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[01:12:28] -- The CXX compiler identification is GNU 6.3.0
[01:12:28] -- The ASM compiler identification is GNU
[01:12:28] -- Found assembler: /usr/local/bin/x86_64-linux-musl-gcc
[01:12:28] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
---
[01:51:35] [ 71%] Linking CXX static library ../../../libLLVMPowerPCInfo.a
[01:51:36] [ 71%] Built target LLVMPowerPCInfo
[01:51:36] Scanning dependencies of target LLVMSystemZCodeGen
[01:51:36] [ 71%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZAsmPrinter.cpp.o
[01:51:37] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp: In member function 'virtual bool {anonymous}::MipsInstructionSelector::select(llvm::MachineInstr&, llvm::CodeGenCoverage&) const':
[01:51:37] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp:296:16: warning: types may not be defined in a for-range-declaration
[01:51:37]      for (const struct Instr &Instruction : Instructions) {
[01:51:37] [ 71%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
[01:51:38] [ 71%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCMCExpr.cpp.o
[01:51:41] [ 71%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCPredicates.cpp.o
[01:51:41] [ 71%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCMachObjectWriter.cpp.o
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[02:45:26] 
[02:45:26] running 119 tests
[02:45:57] .iiiii..ii.....i..i...i..i.i..i.ii..ii....ii.ii....i..........iiii..........i...ii..ii.......ii.i.ii 100/119
[02:46:02] i.....iiiiii.....ii
[02:46:02] 
[02:46:02]  finished in 35.441
[02:46:02] travis_fold:end:test_debuginfo

