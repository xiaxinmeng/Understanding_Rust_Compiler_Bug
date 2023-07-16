plain
travis_time:end:1da19bae:start=1546722470269631165,finish=1546722472382651323,duration=2113020158
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:24] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:24]  ---> Running in 72fe215ded3f
[00:01:24] + TARGET=x86_64-linux-musl
[00:01:24] + ARCH=x86_64
[00:01:24] + OUTPUT=/usr/local
[00:01:24] + shift
[00:01:24] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:24] Cloning into 'musl-cross-make'...
[00:01:24] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:24] 
[00:01:24] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:24] changes and commit them, and you can discard any commits you make in this
[00:01:24] state without impacting any branches by performing another checkout.
[00:01:24] 
[00:01:24] If you want to create a new branch to retain commits you create, you may
[00:01:24] do so (now or later) by using -b with the checkout command again. Example:
[00:01:24] 
[00:01:24]   git checkout -b <new-branch-name>
[00:01:24] + cd musl-cross-make
[00:01:24] ++ nproc
[00:01:24] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:24] + set +x
---
[00:10:55] Sat Jan 5 21:18:58 UTC 2019 - building ...
[00:11:25] Sat Jan 5 21:19:28 UTC 2019 - building ...
[00:11:55] Sat Jan 5 21:19:58 UTC 2019 - building ...
[00:12:25] Sat Jan 5 21:20:28 UTC 2019 - building ...
[00:12:31] musl-toolchain.sh: line 13:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:31] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:46] /build
[00:12:46] musl-toolchain.sh: line 13:  6348 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:46] + cd -
[00:12:46] + cd -
[00:12:46] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:46] + echo /usr/local/x86_64-linux-musl/lib
[00:12:46] + export CC=x86_64-linux-musl-gcc
[00:12:46] + CC=x86_64-linux-musl-gcc
[00:12:46] + export CXX=x86_64-linux-musl-g++
[00:12:46] + CXX=x86_64-linux-musl-g++
[00:12:46] + '[' '!' -d libunwind-release_60 ']'
[00:12:46] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:12:46] + tar xzf -
[00:12:46]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:12:55] 
100   97k    0   97k    0     0   145k      0 --:--:-- --:--:-- --:--:--  145k
[00:12:55] + mkdir libunwind-build
[00:12:55] + cd libunwind-build
[00:12:55] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:12:55] -- The CXX compiler identification is GNU 6.3.0
[00:12:55] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:12:55] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:12:55] -- Detecting C compiler ABI info
---
[00:15:49] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:49]  ---> Running in 736cbe84ffe7
[00:15:49] Removing intermediate container 736cbe84ffe7
[00:15:49]  ---> 0dbd46f1dcdd
[00:15:49] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:49] Removing intermediate container 2bb4c9a7ff71
[00:15:49]  ---> 5b12814127ba
[00:15:49] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:49]  ---> Running in 5e06fe423885
[00:15:49]  ---> Running in 5e06fe423885
[00:15:49] Removing intermediate container 5e06fe423885
[00:15:49]  ---> 5d793ef369e0
[00:15:49] Successfully built 5d793ef369e0
[00:15:49] Successfully tagged rust-ci:latest
[00:15:49] Built container sha256:5d793ef369e0ed4b41c159eddcadcd86340a2fc17a67d39d3a3169813bd9ecbc
[00:15:49] Uploading finished image to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af
[00:22:14] upload failed: - to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af Unable to locate credentials

[00:22:15] travis_time:end:00c4e9c1:start=1546722491950290493,finish=1546723817684314053,duration=1325734023560
[CI_JOB_NAME=dist-x86_64-musl]
[00:22:15] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:49:07]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:49:07]    Compiling rustc-demangle v0.1.10
[00:49:08]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:49:14]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:49:38] error: cannot produce dylib for `arena v0.0.0 (/checkout/src/libarena)` as the target `x86_64-unknown-linux-musl` does not support these crate types
[00:49:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:49:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --host x86_64-unknown-linux-musl --target x86_64-unknown-linux-musl
[00:49:38] Build completed unsuccessfully in 0:25:40
[00:49:38] Build completed unsuccessfully in 0:25:40
/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2158e461
$ dmesg | grep -i kill
