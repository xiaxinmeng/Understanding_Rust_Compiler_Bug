plain
travis_time:end:00ec9dd1:start=1544125644195318850,finish=1544125700139917296,duration=55944598446
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:04] Step 5/11 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:04]  ---> Running in 2554b80d8bea
[00:01:05] + TARGET=x86_64-linux-musl
[00:01:05] + ARCH=x86_64
[00:01:05] + OUTPUT=/usr/local
[00:01:05] + shift
[00:01:05] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:05] Cloning into 'musl-cross-make'...
[00:01:05] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:05] 
[00:01:05] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:05] changes and commit them, and you can discard any commits you make in this
[00:01:05] state without impacting any branches by performing another checkout.
[00:01:05] 
[00:01:05] If you want to create a new branch to retain commits you create, you may
[00:01:05] do so (now or later) by using -b with the checkout command again. Example:
[00:01:05] 
[00:01:05]   git checkout -b <new-branch-name>
[00:01:05] + cd musl-cross-make
[00:01:05] ++ nproc
[00:01:05] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:05] + set +x
---
[00:09:05] Thu Dec 6 19:57:36 UTC 2018 - building ...
[00:09:35] Thu Dec 6 19:58:06 UTC 2018 - building ...
[00:10:05] Thu Dec 6 19:58:36 UTC 2018 - building ...
[00:10:36] Thu Dec 6 19:59:06 UTC 2018 - building ...
[00:11:04] musl-toolchain.sh: line 13:    19 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:04] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:11:16] musl-toolchain.sh: line 13:  6315 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:16] + cd ..
[00:11:16] + cd ..
[00:11:16] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:11:16] + echo /usr/local/x86_64-linux-musl/lib
[00:11:16] + export CC=x86_64-linux-musl-gcc
[00:11:16] + CC=x86_64-linux-musl-gcc
[00:11:16] + export CXX=x86_64-linux-musl-g++
[00:11:16] + CXX=x86_64-linux-musl-g++
[00:11:16] + '[' '!' -d libunwind-release_60 ']'
[00:11:16] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:11:16] + tar xzf -
[00:11:16]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:11:24] 
100   97k    0   97k    0     0   138k      0 --:--:-- --:--:-- --:--:--  138k
[00:11:24] + mkdir libunwind-build
[00:11:24] + cd libunwind-build
[00:11:24] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:11:24] -- The CXX compiler identification is GNU 6.3.0
[00:11:24] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:11:24] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:11:24] -- Detecting C compiler ABI info
---
[00:14:22] Step 8/11 : ENV RUST_CONFIGURE_ARGS --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:14:22]  ---> Running in 57b92a1fda93
[00:14:22]  ---> efa46f766c4f
[00:14:22] Removing intermediate container 57b92a1fda93
[00:14:22] Step 9/11 : ENV HOSTS x86_64-unknown-linux-musl CC_x86_64_unknown_linux_musl x86_64-linux-musl-gcc CXX_x86_64_unknown_linux_musl x86_64-linux-musl-g++
[00:14:22]  ---> 748d182bfa87
[00:14:22] Removing intermediate container 6502df8fccfe
[00:14:22] Removing intermediate container 6502df8fccfe
[00:14:22] Step 10/11 : ENV RUSTFLAGS "-C target-feature=-crt-static"
[00:14:22]  ---> 325973d5d18f
[00:14:22] Removing intermediate container eeaf536a6352
[00:14:22] Step 11/11 : ENV SCRIPT python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:14:22]  ---> Running in 8d4fbf2501d1
[00:14:22]  ---> Running in 8d4fbf2501d1
[00:14:22]  ---> 83a5fa57300e
[00:14:22] Removing intermediate container 8d4fbf2501d1
[00:14:22] Successfully built 83a5fa57300e
[00:14:22] Successfully tagged rust-ci:latest
[00:14:22] Built container sha256:83a5fa57300eb905e3d3bf93983b3a15b724402a32c426d11aac15f2c89e2aa4
[00:14:22] Uploading finished image to s3://rust-lang-ci-sccache2/docker/9b40357f03d2ff7f59cf844c7bf6246c6ca2d118ff86325814424d031978b11b73ecfb8239f192f31a514b43e21494eb71a2581284f477995b7e5da58f4f6ec6
[00:19:32] upload failed: - to s3://rust-lang-ci-sccache2/docker/9b40357f03d2ff7f59cf844c7bf6246c6ca2d118ff86325814424d031978b11b73ecfb8239f192f31a514b43e21494eb71a2581284f477995b7e5da58f4f6ec6 Unable to locate credentials

[00:19:32] travis_time:end:229757e2:start=1544125722701506325,finish=1544126883350305813,duration=1160648799488
[CI_JOB_NAME=dist-x86_64-musl]
[00:19:32] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:23:51]    Compiling crossbeam-deque v0.2.0
[00:23:54]    Compiling rustc-rayon v0.1.1
[00:23:57]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:24:00]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:24:00] error[E0433]: failed to resolve. Use of undeclared type or module `RelroLevel`
[00:24:00]   --> src/librustc_target/spec/linux_musl_base.rs:49:24
[00:24:00]    |
[00:24:00] 49 |     base.relro_level = RelroLevel::Full;
[00:24:00]    |                        ^^^^^^^^^^ Use of undeclared type or module `RelroLevel`
[00:24:01] error: aborting due to previous error
[00:24:01] 
[00:24:01] For more information about this error, try `rustc --explain E0433`.
[00:24:01] error: Could not compile `rustc_target`.
