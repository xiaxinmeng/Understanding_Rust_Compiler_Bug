plain
travis_time:end:1112fb90:start=1551376132628337909,finish=1551376206664970894,duration=74036632985
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:32] Step 5/11 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:32]  ---> Running in cfdc351003ce
[00:01:32] + TARGET=x86_64-linux-musl
[00:01:32] + ARCH=x86_64
[00:01:32] + OUTPUT=/usr/local
[00:01:32] + shift
[00:01:32] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:32] Cloning into 'musl-cross-make'...
[00:01:32] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:32] 
[00:01:32] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:32] changes and commit them, and you can discard any commits you make in this
[00:01:32] state without impacting any branches by performing another checkout.
[00:01:32] 
[00:01:32] If you want to create a new branch to retain commits you create, you may
[00:01:32] do so (now or later) by using -b with the checkout command again. Example:
[00:01:32] 
[00:01:32]   git checkout -b <new-branch-name>
[00:01:33] + cd musl-cross-make
[00:01:33] ++ nproc
[00:01:33] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:33] + set +x
---
[00:10:33] Thu Feb 28 18:00:49 UTC 2019 - building ...
[00:11:03] Thu Feb 28 18:01:19 UTC 2019 - building ...
[00:11:33] Thu Feb 28 18:01:49 UTC 2019 - building ...
[00:12:03] Thu Feb 28 18:02:19 UTC 2019 - building ...
[00:12:17] musl-toolchain.sh: line 3:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:17] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:32] /build
[00:12:32] musl-toolchain.sh: line 3:  6304 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:32] + cd -
[00:12:32] + cd -
[00:12:32] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:32] + echo /usr/local/x86_64-linux-musl/lib
[00:12:32] + export CC=x86_64-linux-musl-gcc
[00:12:32] + CC=x86_64-linux-musl-gcc
[00:12:32] + export CXX=x86_64-linux-musl-g++
[00:12:32] + CXX=x86_64-linux-musl-g++
[00:12:32] + '[' '!' -d libunwind-release_70 ']'
[00:12:32] + curl -L https://github.com/llvm-mirror/llvm/archive/release_70.tar.gz
[00:12:32] + tar xzf -
[00:12:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:15:39] Step 8/11 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:39]  ---> Running in b6e8aadbdd9f
[00:15:39] Removing intermediate container b6e8aadbdd9f
[00:15:39]  ---> c52c70957def
[00:15:39] Step 9/11 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:39] Removing intermediate container 4a166936692b
[00:15:39]  ---> dab3ca49e7f4
[00:15:39]  ---> dab3ca49e7f4
[00:15:39] Step 10/11 : ENV RUSTFLAGS="-C target-feature=-crt-static"
[00:15:39] Removing intermediate container 0f2a3d9a43e0
[00:15:39]  ---> eacc49aac209
[00:15:39] Step 11/11 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:39]  ---> Running in 32b1d3ee1b15
[00:15:39]  ---> Running in 32b1d3ee1b15
[00:15:39] Removing intermediate container 32b1d3ee1b15
[00:15:39]  ---> bc2a6cc7f0d0
[00:15:39] Successfully built bc2a6cc7f0d0
[00:15:39] Successfully tagged rust-ci:latest
[00:15:39] Built container sha256:bc2a6cc7f0d0fd7caccbe1df84786cae3dee0e0137014f9340bb943a1dd2c7ca
[00:15:39] Uploading finished image to s3://rust-lang-ci-sccache2/docker/fd3426e13b5752ae462db71e06d5b48945a4ba81170afb8377771e78b27939ac4e9cf4cf1a6079d885d0e41b8cb27e1d6b2c067dccae190830ff6799b32ece21
[00:22:04] upload failed: - to s3://rust-lang-ci-sccache2/docker/fd3426e13b5752ae462db71e06d5b48945a4ba81170afb8377771e78b27939ac4e9cf4cf1a6079d885d0e41b8cb27e1d6b2c067dccae190830ff6799b32ece21 Unable to locate credentials

[00:22:04] travis_time:end:1545ec80:start=1551376240281674137,finish=1551377539948824997,duration=1299667150860
[CI_JOB_NAME=dist-x86_64-musl]
[00:22:04] [CI_JOB_NAME=dist-x86_64-musl]
---

[01:09:55] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[01:09:55] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-linux-musl-gcc" "-DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[01:09:55] -- The CXX compiler identification is GNU 6.3.0
[01:09:55] -- The ASM compiler identification is GNU
[01:09:55] -- Found assembler: /usr/local/bin/x86_64-linux-musl-gcc
[01:09:55] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
---
[01:48:28] [ 73%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsInstructionSelector.cpp.o
[01:48:33] [ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCMIPeephole.cpp.o
[01:48:33] [ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelDAGToDAG.cpp.o
[01:48:33] [ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelLowering.cpp.o
[01:48:40] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp: In member function 'virtual bool {anonymous}::MipsInstructionSelector::select(llvm::MachineInstr&, llvm::CodeGenCoverage&) const':
[01:48:40] /checkout/src/llvm-project/llvm/lib/Target/Mips/MipsInstructionSelector.cpp:296:16: warning: types may not be defined in a for-range-declaration
[01:48:40]      for (const struct Instr &Instruction : Instructions) {
[01:48:40] [ 73%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
[01:48:42] [ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCRegisterInfo.cpp.o
[01:48:44] [ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZInstrInfo.cpp.o
[01:48:50] [ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZLDCleanup.cpp.o
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-musl -> x86_64-unknown-linux-musl)
[02:39:21] 
[02:39:21] running 119 tests
[02:39:51] .iiiii..ii.....i..i...i..i.i..i.ii..ii....ii.ii....i..........iiii..........i...ii..ii.......ii.i.ii 100/119
[02:39:55] i.....iiiiii.....ii
[02:39:55] 
[02:39:55]  finished in 34.208
[02:39:55] travis_fold:end:test_debuginfo

---
[02:52:08]    Compiling arena v0.0.0 (/checkout/src/libarena)
[02:52:08]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[02:52:17]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[02:53:38]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
The job exceeded the maximum time limit for jobs, and has been terminated.
