plain
travis_time:end:0622a888:start=1551704478109967134,finish=1551704550138746737,duration=72028779603
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:52] Step 5/12 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:52]  ---> Running in b00c3859722f
[00:01:52] + TARGET=x86_64-linux-musl
[00:01:52] + ARCH=x86_64
[00:01:52] + OUTPUT=/usr/local
[00:01:52] + shift
[00:01:52] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:52] Cloning into 'musl-cross-make'...
[00:01:53] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:53] 
[00:01:53] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:53] changes and commit them, and you can discard any commits you make in this
[00:01:53] state without impacting any branches by performing another checkout.
[00:01:53] 
[00:01:53] If you want to create a new branch to retain commits you create, you may
[00:01:53] do so (now or later) by using -b with the checkout command again. Example:
[00:01:53] 
[00:01:53]   git checkout -b <new-branch-name>
[00:01:53] + cd musl-cross-make
[00:01:53] ++ nproc
[00:01:53] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:53] + set +x
---
[00:09:53] Mon Mar 4 13:12:31 UTC 2019 - building ...
[00:10:23] Mon Mar 4 13:13:01 UTC 2019 - building ...
[00:10:53] Mon Mar 4 13:13:31 UTC 2019 - building ...
[00:11:23] Mon Mar 4 13:14:01 UTC 2019 - building ...
[00:11:49] musl-toolchain.sh: line 3:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:49] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:02] /build
[00:12:02] musl-toolchain.sh: line 3:  6326 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:02] + cd -
[00:12:02] + cd -
[00:12:02] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:02] + echo /usr/local/x86_64-linux-musl/lib
[00:12:02] + export CC=x86_64-linux-musl-gcc
[00:12:02] + CC=x86_64-linux-musl-gcc
[00:12:02] + export CXX=x86_64-linux-musl-g++
[00:12:02] + CXX=x86_64-linux-musl-g++
[00:12:02] + '[' '!' -d libunwind-release_70 ']'
[00:12:02] + curl -L https://github.com/llvm-mirror/llvm/archive/release_70.tar.gz
[00:12:02] + tar xzf -
[00:12:02]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:15:10] Step 9/12 : ENV CFLAGS_x86_64_unknown_linux_musl=-Wa,-mrelax-relocations=no
[00:15:10]  ---> Running in af3b124d6a79
[00:15:10] Removing intermediate container af3b124d6a79
[00:15:10]  ---> 8d19a82c4967
[00:15:10] Step 10/12 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:10] Removing intermediate container 5001adba8c67
[00:15:10]  ---> f15e07768f17
[00:15:10]  ---> f15e07768f17
[00:15:10] Step 11/12 : ENV RUSTFLAGS="-C target-feature=-crt-static"
[00:15:11] Removing intermediate container a7a0747d24af
[00:15:11]  ---> bc1fb5fd0c82
[00:15:11] Step 12/12 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:11]  ---> Running in 221674a2d73f
[00:15:11]  ---> Running in 221674a2d73f
[00:15:11] Removing intermediate container 221674a2d73f
[00:15:11]  ---> f291f47579ca
[00:15:11] Successfully built f291f47579ca
[00:15:11] Successfully tagged rust-ci:latest
[00:15:11] Built container sha256:f291f47579ca55c9bd4847ab3ff0de9b31801509ec7ed92987f4df313b4ffcdb
[00:15:11] Uploading finished image to s3://rust-lang-ci-sccache2/docker/978bd17ef1550a2c52f85510dfb0d006c2ae5e1818e57ed2b1531f3909839c0792a345d582279af43e931c354ebf9958ca4ef17416515fdd2daf6a9891214877
[00:21:34] upload failed: - to s3://rust-lang-ci-sccache2/docker/978bd17ef1550a2c52f85510dfb0d006c2ae5e1818e57ed2b1531f3909839c0792a345d582279af43e931c354ebf9958ca4ef17416515fdd2daf6a9891214877 Unable to locate credentials

[00:21:35] travis_time:end:2eca1678:start=1551704613232083101,finish=1551705852953282855,duration=1239721199754
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:35] [CI_JOB_NAME=dist-x86_64-musl]
---

[01:10:33] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[01:10:33] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-linux-musl-gcc" "-DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -Wa,-mrelax-relocations=no -m64 -static" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -Wa,-mrelax-relocations=no -m64 -static -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[01:10:33] -- The CXX compiler identification is GNU 6.3.0
[01:10:33] -- The ASM compiler identification is GNU
[01:10:33] -- Found assembler: /usr/local/bin/x86_64-linux-musl-gcc
[01:10:33] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
---
[01:46:07] [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsInstructionSelector.cpp.o
[01:46:11] [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
[01:46:12] [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[01:46:13] [ 69%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCCCState.cpp.o
[01:46:19] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp: In member function 'virtual bool {anonymous}::MipsInstructionSelector::select(llvm::MachineInstr&, llvm::CodeGenCoverage&) const':
[01:46:19] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp:296:16: warning: types may not be defined in a for-range-declaration
[01:46:19]      for (const struct Instr &Instruction : Instructions) {
[01:46:19] [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelLowering.cpp.o
[01:46:21] [ 69%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCCTRLoops.cpp.o
[01:46:21] [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFrameLowering.cpp.o
[01:46:22] [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[02:40:18] 
[02:40:18] running 119 tests
[02:40:46] .iiiii..ii.....i..i...i..i.i..i.ii..ii....ii.ii....i..........iiii..........i...ii..ii.......ii.i.ii 100/119
[02:40:51] i.....iiiiii.....ii
[02:40:51] 
[02:40:51]  finished in 32.319
[02:40:51] travis_fold:end:test_debuginfo

