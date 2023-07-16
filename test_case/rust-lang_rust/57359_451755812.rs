plain
travis_time:end:02793216:start=1546790133342827152,finish=1546790203066297101,duration=69723469949
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:05] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:05]  ---> Running in c167f5bc3d12
[00:01:05] + TARGET=x86_64-linux-musl
[00:01:05] + ARCH=x86_64
[00:01:05] + OUTPUT=/usr/local
[00:01:05] + shift
[00:01:05] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:05] Cloning into 'musl-cross-make'...
[00:01:06] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:06] 
[00:01:06] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:06] changes and commit them, and you can discard any commits you make in this
[00:01:06] state without impacting any branches by performing another checkout.
[00:01:06] 
[00:01:06] If you want to create a new branch to retain commits you create, you may
[00:01:06] do so (now or later) by using -b with the checkout command again. Example:
[00:01:06] 
[00:01:06]   git checkout -b <new-branch-name>
[00:01:06] + cd musl-cross-make
[00:01:06] ++ nproc
[00:01:06] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:06] + set +x
---
[00:09:36] Sun Jan 6 16:06:27 UTC 2019 - building ...
[00:10:06] Sun Jan 6 16:06:57 UTC 2019 - building ...
[00:10:36] Sun Jan 6 16:07:27 UTC 2019 - building ...
[00:11:06] Sun Jan 6 16:07:57 UTC 2019 - building ...
[00:11:34] musl-toolchain.sh: line 13:    21 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:34] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:11:47] musl-toolchain.sh: line 13:  6337 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:47] + cd -
[00:11:47] + cd -
[00:11:47] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:11:47] + echo /usr/local/x86_64-linux-musl/lib
[00:11:47] + export CC=x86_64-linux-musl-gcc
[00:11:47] + CC=x86_64-linux-musl-gcc
[00:11:47] + export CXX=x86_64-linux-musl-g++
---
[00:11:56] 
100   97k    0   97k    0     0   141k      0 --:--:-- --:--:-- --:--:--  141k
[00:11:56] + mkdir libunwind-build
[00:11:56] + cd libunwind-build
[00:11:56] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:11:56] -- The CXX compiler identification is GNU 6.3.0
[00:11:56] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:11:56] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:11:56] -- Detecting C compiler ABI info
---
[00:14:50] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:14:50]  ---> Running in 614dbac93555
[00:14:50] Removing intermediate container 614dbac93555
[00:14:50]  ---> 3b8d978a6895
[00:14:50] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:14:50] Removing intermediate container 4f14e3b0c58f
[00:14:50]  ---> 48a063d37e38
[00:14:50] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:14:50]  ---> Running in dea5c2c98656
[00:14:50]  ---> Running in dea5c2c98656
[00:14:50] Removing intermediate container dea5c2c98656
[00:14:50]  ---> 12188a95b344
[00:14:50] Successfully built 12188a95b344
[00:14:50] Successfully tagged rust-ci:latest
[00:14:50] Built container sha256:12188a95b344a864c0ba1cb95de39f9c3c6b6c89c5527ae466b3f74f46a34cd5
[00:14:50] Uploading finished image to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af
[00:21:11] upload failed: - to s3://rust-lang-ci-sccache2/docker/429b0a8dd3090b6b3cb0931f9f35e54ed3fd23ccbe6e6710e293d930db79c504a2a7b69b6aba87b6ddbeb0fae31b7fd7d2edcf63b27e3ee7638e9d4f6418b6af Unable to locate credentials

[00:21:12] travis_time:end:03657f71:start=1546790219164751084,finish=1546791482969973270,duration=1263805222186
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:12] [CI_JOB_NAME=dist-x86_64-musl]
