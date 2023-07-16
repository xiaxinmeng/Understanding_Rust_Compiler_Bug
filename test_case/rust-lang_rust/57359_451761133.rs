plain
travis_time:end:1f5812f4:start=1546794623136873133,finish=1546794702005233289,duration=78868360156
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:09] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:09]  ---> Running in 968c187fd3c6
[00:01:09] + TARGET=x86_64-linux-musl
[00:01:09] + ARCH=x86_64
[00:01:09] + OUTPUT=/usr/local
[00:01:09] + shift
[00:01:09] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:09] Cloning into 'musl-cross-make'...
[00:01:09] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:09] 
[00:01:09] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:09] changes and commit them, and you can discard any commits you make in this
[00:01:09] state without impacting any branches by performing another checkout.
[00:01:09] 
[00:01:09] If you want to create a new branch to retain commits you create, you may
[00:01:09] do so (now or later) by using -b with the checkout command again. Example:
[00:01:09] 
[00:01:09]   git checkout -b <new-branch-name>
[00:01:09] + cd musl-cross-make
[00:01:09] ++ nproc
[00:01:09] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:09] + set +x
---
[00:10:10] Sun Jan 6 17:22:00 UTC 2019 - building ...
[00:10:40] Sun Jan 6 17:22:30 UTC 2019 - building ...
[00:11:10] Sun Jan 6 17:23:00 UTC 2019 - building ...
[00:11:40] Sun Jan 6 17:23:30 UTC 2019 - building ...
[00:12:04] musl-toolchain.sh: line 13:    21 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:04] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:18] /build
[00:12:18] musl-toolchain.sh: line 13:  6310 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:18] + cd -
[00:12:18] + cd -
[00:12:18] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:18] + echo /usr/local/x86_64-linux-musl/lib
[00:12:18] + export CC=x86_64-linux-musl-gcc
[00:12:18] + CC=x86_64-linux-musl-gcc
[00:12:18] + export CXX=x86_64-linux-musl-g++
[00:12:18] + CXX=x86_64-linux-musl-g++
[00:12:18] + '[' '!' -d libunwind-release_60 ']'
[00:12:18] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:12:18] + tar xzf -
[00:12:18]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:12:27] 
100   97k    0   97k    0     0   128k      0 --:--:-- --:--:-- --:--:--  128k
[00:12:27] + mkdir libunwind-build
[00:12:27] + cd libunwind-build
[00:12:27] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:12:27] -- The CXX compiler identification is GNU 6.3.0
[00:12:27] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:12:27] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:12:27] -- Detecting C compiler ABI info
---
[00:15:20] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:21]  ---> Running in 985aea2209ed
[00:15:21] Removing intermediate container 985aea2209ed
[00:15:21]  ---> 1c57f6623539
[00:15:21] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:21] Removing intermediate container 91156edc2577
[00:15:21]  ---> 28df4ad0c632
[00:15:21] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:21]  ---> Running in db97a9834a63
[00:15:21]  ---> Running in db97a9834a63
[00:15:21] Removing intermediate container db97a9834a63
[00:15:21]  ---> 325d597cc124
[00:15:21] Successfully built 325d597cc124
[00:15:21] Successfully tagged rust-ci:latest
[00:15:21] Built container sha256:325d597cc1249dde8231000e43db2bb2d697799a7920181dc8cf687637f34eda
[00:15:21] Uploading finished image to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af
[00:21:50] upload failed: - to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af Unable to locate credentials

[00:21:50] travis_time:end:15d681ac:start=1546794721548095684,finish=1546796020419764872,duration=1298871669188
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:50] [CI_JOB_NAME=dist-x86_64-musl]
