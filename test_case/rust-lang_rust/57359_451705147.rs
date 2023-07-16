plain
travis_time:end:05476d2e:start=1546727764124139164,finish=1546727766384264666,duration=2260125502
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:21] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:21]  ---> Running in a463faceaa7c
[00:01:22] + TARGET=x86_64-linux-musl
[00:01:22] + ARCH=x86_64
[00:01:22] + OUTPUT=/usr/local
[00:01:22] + shift
[00:01:22] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:22] Cloning into 'musl-cross-make'...
[00:01:22] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:22] 
[00:01:22] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:22] changes and commit them, and you can discard any commits you make in this
[00:01:22] state without impacting any branches by performing another checkout.
[00:01:22] 
[00:01:22] If you want to create a new branch to retain commits you create, you may
[00:01:22] do so (now or later) by using -b with the checkout command again. Example:
[00:01:22] 
[00:01:22]   git checkout -b <new-branch-name>
[00:01:22] + cd musl-cross-make
[00:01:22] ++ nproc
[00:01:22] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:22] + set +x
---
[00:09:53] Sat Jan 5 22:46:09 UTC 2019 - building ...
[00:10:23] Sat Jan 5 22:46:39 UTC 2019 - building ...
[00:10:53] Sat Jan 5 22:47:09 UTC 2019 - building ...
[00:11:23] Sat Jan 5 22:47:39 UTC 2019 - building ...
[00:11:48] musl-toolchain.sh: line 13:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:48] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:02] musl-toolchain.sh: line 13:  6327 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:02] + cd -
[00:12:02] + cd -
[00:12:02] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:02] + echo /usr/local/x86_64-linux-musl/lib
[00:12:02] + export CC=x86_64-linux-musl-gcc
[00:12:02] + CC=x86_64-linux-musl-gcc
[00:12:02] + export CXX=x86_64-linux-musl-g++
---
[00:12:11] 
100   97k    0   97k    0     0   131k      0 --:--:-- --:--:-- --:--:--  131k
[00:12:11] + mkdir libunwind-build
[00:12:11] + cd libunwind-build
[00:12:11] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:12:11] -- The CXX compiler identification is GNU 6.3.0
[00:12:11] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:12:11] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:12:11] -- Detecting C compiler ABI info
---
[00:15:05] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:05]  ---> Running in 0c5f57ea02eb
[00:15:05] Removing intermediate container 0c5f57ea02eb
[00:15:05]  ---> 69d6715c2a2d
[00:15:05] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:05] Removing intermediate container c8dd61953657
[00:15:05]  ---> 48823bea784e
[00:15:05] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:05]  ---> Running in d6e3c3b68508
[00:15:05]  ---> Running in d6e3c3b68508
[00:15:05] Removing intermediate container d6e3c3b68508
[00:15:05]  ---> 99624e006d65
[00:15:05] Successfully built 99624e006d65
[00:15:05] Successfully tagged rust-ci:latest
[00:15:05] Built container sha256:99624e006d6517b40351ed5652cd978cca95974665b454edd80896c6db835c88
[00:15:05] Uploading finished image to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af
[00:21:26] upload failed: - to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af Unable to locate credentials

[00:21:26] travis_time:end:00872f97:start=1546727784454032189,finish=1546729062258686526,duration=1277804654337
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:26] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:23:38]   Downloaded fixedbitset v0.1.9
[00:23:58]   Downloaded ordermap v0.3.5
[00:23:58]   Downloaded filetime v0.2.4
[00:23:58]   Downloaded cfg-if v0.1.6
[00:24:31] warning: spurious network error (2 tries remaining): [28] Timeout was reached (failed to download any data for `unicode-xid v0.1.0` within 30s)
[00:25:30] warning: spurious network error (1 tries remaining): [28] Timeout was reached (failed to download any data for `unicode-xid v0.1.0` within 30s)
[00:25:38]    Compiling proc-macro2 v0.4.24
[00:25:38]    Compiling unicode-xid v0.1.0
[00:25:38]    Compiling serde v1.0.82
[00:25:38]    Compiling ryu v0.2.7
---
[01:47:04] [ 72%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZExpandPseudo.cpp.o
[01:47:04] [ 72%] Built target LLVMSystemZDisassembler
[01:47:04] Scanning dependencies of target LLVMSystemZAsmPrinter
[01:47:04] [ 72%] Building CXX object lib/Target/SystemZ/InstPrinter/CMakeFiles/LLVMSystemZAsmPrinter.dir/SystemZInstPrinter.cpp.o
[01:47:04] /checkout/src/llvm/lib/Target/Mips/MipsInstructionSelector.cpp: In member function 'virtual bool {anonymous}::MipsInstructionSelector::select(llvm::MachineInstr&, llvm::CodeGenCoverage&) const':
[01:47:04] /checkout/src/llvm/lib/Target/Mips/MipsInstructionSelector.cpp:260:16: warning: types may not be defined in a for-range-declaration
[01:47:04]      for (const struct Instr &Instruction : Instructions) {
[01:47:04] [ 73%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
[01:47:06] [ 73%] Linking CXX static library ../../../libLLVMSystemZAsmPrinter.a
[01:47:06] [ 73%] Built target LLVMSystemZAsmPrinter
[01:47:06] Scanning dependencies of target LLVMSystemZDesc
---
[02:03:46] 
[02:04:00]    Compiling test v0.0.0 (/checkout/src/libtest)
[02:04:00] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-linux-musl`
[02:04:00] 
[02:04:09] error: cannot produce dylib for `arena v0.0.0 (/checkout/src/libarena)` as the target `x86_64-unknown-linux-musl` does not support these crate types
[02:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[02:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --host x86_64-unknown-linux-musl --target x86_64-unknown-linux-musl
[02:04:09] Build completed unsuccessfully in 1:37:27
_64-unknown-linux-gnu/release/deps
217432 ./obj/build/x86_64-unknown-linux-musl/stage2
---
149084 ./obj/build/x86_64-unknown-linux-gnu/stage1-codegen/x86_64-unknown-linux-musl
149080 ./obj/build/x86_64-unknown-linux-gnu/stage1-codegen/x86_64-unknown-linux-musl/release
148292 ./obj/build/x86_64-unknown-linux-gnu/stage1-codegen/x86_64-unknown-linux-musl/release/deps
144256 ./obj/build/bootstrap/debug/incremental/bootstrap-2pq9xsgmnli7u
144252 ./obj/build/bootstrap/debug/incremental/bootstrap-2pq9xsgmnli7u/s-f89pnbk7nf-cmh2vv-21p7sgjt03yy
121936 ./obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib
121932 ./obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl
120476 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
119444 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
118108 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
115344 ./src/llvm/test/CodeGen
