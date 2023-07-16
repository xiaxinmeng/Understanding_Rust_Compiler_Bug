plain
travis_time:end:058c8efd:start=1544119843510274354,finish=1544119908932540163,duration=65422265809
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:16] Step 5/11 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:16]  ---> Running in 058c6df8f543
[00:01:16] + TARGET=x86_64-linux-musl
[00:01:16] + ARCH=x86_64
[00:01:16] + OUTPUT=/usr/local
[00:01:16] + shift
[00:01:16] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:16] Cloning into 'musl-cross-make'...
[00:01:17] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:17] 
[00:01:17] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:17] changes and commit them, and you can discard any commits you make in this
[00:01:17] state without impacting any branches by performing another checkout.
[00:01:17] 
[00:01:17] If you want to create a new branch to retain commits you create, you may
[00:01:17] do so (now or later) by using -b with the checkout command again. Example:
[00:01:17] 
[00:01:17]   git checkout -b <new-branch-name>
[00:01:17] + cd musl-cross-make
[00:01:17] ++ nproc
[00:01:17] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:17] + set +x
---
[00:10:17] Thu Dec 6 18:22:15 UTC 2018 - building ...
[00:10:47] Thu Dec 6 18:22:45 UTC 2018 - building ...
[00:11:17] Thu Dec 6 18:23:15 UTC 2018 - building ...
[00:11:47] Thu Dec 6 18:23:45 UTC 2018 - building ...
[00:11:52] musl-toolchain.sh: line 13:    18 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:52] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:06] musl-toolchain.sh: line 13:  6376 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:06] + cd ..
[00:12:06] + cd ..
[00:12:06] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:06] + echo /usr/local/x86_64-linux-musl/lib
[00:12:06] + export CC=x86_64-linux-musl-gcc
[00:12:06] + CC=x86_64-linux-musl-gcc
[00:12:06] + export CXX=x86_64-linux-musl-g++
[00:12:06] + CXX=x86_64-linux-musl-g++
[00:12:06] + '[' '!' -d libunwind-release_60 ']'
[00:12:06] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:12:06] + tar xzf -
[00:12:06]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   97k    0   97k    0     0   132k      0 --:--:-- --:--:-- --:--:-- 1028k
[00:12:15] + mkdir libunwind-build
[00:12:15] + cd libunwind-build
[00:12:15] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:12:15] -- The CXX compiler identification is GNU 6.3.0
[00:12:15] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:12:15] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:12:15] -- Detecting C compiler ABI info
---
[00:15:15] Step 8/11 : ENV RUST_CONFIGURE_ARGS --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:15]  ---> Running in a9a0a3016153
[00:15:15]  ---> e1640d38ab65
[00:15:15] Removing intermediate container a9a0a3016153
[00:15:15] Step 9/11 : ENV HOSTS x86_64-unknown-linux-musl CC_x86_64_unknown_linux_musl x86_64-linux-musl-gcc CXX_x86_64_unknown_linux_musl x86_64-linux-musl-g++
[00:15:15]  ---> 25066358f147
[00:15:15] Removing intermediate container f9be13313a86
[00:15:15] Removing intermediate container f9be13313a86
[00:15:15] Step 10/11 : ENV RUSTFLAGS "-C target-feature=-crt-static"
[00:15:15]  ---> 418958a3c3b6
[00:15:15] Removing intermediate container 4171beebbdfe
[00:15:15] Step 11/11 : ENV SCRIPT python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:15]  ---> Running in 6863c0f3f301
[00:15:15]  ---> Running in 6863c0f3f301
[00:15:15]  ---> 37d3b0f26ae3
[00:15:15] Removing intermediate container 6863c0f3f301
[00:15:15] Successfully built 37d3b0f26ae3
[00:15:15] Successfully tagged rust-ci:latest
[00:15:15] Built container sha256:37d3b0f26ae3b0c458e1410d492bf831634b3fa0ae87d0e77f6b3c2958aef6fd
[00:15:15] Uploading finished image to s3://rust-lang-ci-sccache2/docker/9b40357f03d2ff7f59cf844c7bf6246c6ca2d118ff86325814424d031978b11b73ecfb8239f192f31a514b43e21494eb71a2581284f477995b7e5da58f4f6ec6
[00:20:28] upload failed: - to s3://rust-lang-ci-sccache2/docker/9b40357f03d2ff7f59cf844c7bf6246c6ca2d118ff86325814424d031978b11b73ecfb8239f192f31a514b43e21494eb71a2581284f477995b7e5da58f4f6ec6 Unable to locate credentials

[00:20:29] travis_time:end:02ae2f20:start=1544119930660492711,finish=1544121146523891682,duration=1215863398971
[CI_JOB_NAME=dist-x86_64-musl]
[00:20:29] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:25:10]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:25:12] error[E0308]: mismatched types
[00:25:12]   --> src/librustc_target/spec/linux_musl_base.rs:49:24
[00:25:12]    |
[00:25:12] 49 |     base.relro_level = true;
[00:25:12]    |                        ^^^^ expected enum `spec::RelroLevel`, found bool
[00:25:12]    |
[00:25:12]    = note: expected type `spec::RelroLevel`
[00:25:12] 
[00:25:12] error: aborting due to previous error
[00:25:12] 
[00:25:12] For more information about this error, try `rustc --explain E0308`.
[00:25:12] For more information about this error, try `rustc --explain E0308`.
[00:25:12] error: Could not compile `rustc_target`.
[00:25:12] warning: build failed, waiting for other jobs to finish...
_fold:start:after_failure.4
travis_time:start:11dc1460
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
