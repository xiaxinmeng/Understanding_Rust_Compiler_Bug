plain
travis_time:end:004adae0:start=1544143027745501314,finish=1544143081441806596,duration=53696305282
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:05] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:05]  ---> Running in 431c7bdf83fd
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
[00:09:36] Fri Dec 7 00:47:47 UTC 2018 - building ...
[00:10:06] Fri Dec 7 00:48:17 UTC 2018 - building ...
[00:10:36] Fri Dec 7 00:48:47 UTC 2018 - building ...
[00:11:06] Fri Dec 7 00:49:17 UTC 2018 - building ...
[00:11:28] musl-toolchain.sh: line 13:    19 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:28] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:11:41] musl-toolchain.sh: line 13:  6342 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:41] + cd ..
[00:11:41] + cd ..
[00:11:41] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:11:41] + echo /usr/local/x86_64-linux-musl/lib
[00:11:41] + export CC=x86_64-linux-musl-gcc
[00:11:41] + CC=x86_64-linux-musl-gcc
[00:11:41] + export CXX=x86_64-linux-musl-g++
[00:11:41] + CXX=x86_64-linux-musl-g++
[00:11:41] + '[' '!' -d libunwind-release_60 ']'
[00:11:41] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
[00:11:41] + tar xzf -
[00:11:41]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:11:49] 
100   97k    0   97k    0     0   131k      0 --:--:-- --:--:-- --:--:--  131k
[00:11:50] + mkdir libunwind-build
[00:11:50] + cd libunwind-build
[00:11:50] + cmake ../libunwind-release_60 -DLLVM_PATH=/build/llvm-release_60 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ -DCMAKE_C_FLAGS= -DCMAKE_CXX_FLAGS=
[00:11:50] -- The CXX compiler identification is GNU 6.3.0
[00:11:50] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:11:50] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:11:50] -- Detecting C compiler ABI info
---
[00:14:46] Step 8/10 : ENV RUST_CONFIGURE_ARGS --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:14:46]  ---> Running in 499ef819cbd8
[00:14:46]  ---> fd537e8a1499
[00:14:46] Removing intermediate container 499ef819cbd8
[00:14:46] Step 9/10 : ENV HOSTS x86_64-unknown-linux-musl CC_x86_64_unknown_linux_musl x86_64-linux-musl-gcc CXX_x86_64_unknown_linux_musl x86_64-linux-musl-g++
[00:14:46]  ---> 879967b2a1fc
[00:14:46] Removing intermediate container 2ffe4d70b4f6
[00:14:46] Step 10/10 : ENV SCRIPT python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:14:46]  ---> Running in c7bc067b6f70
[00:14:46]  ---> Running in c7bc067b6f70
[00:14:47]  ---> 1dedfc47bf1d
[00:14:47] Removing intermediate container c7bc067b6f70
[00:14:47] Successfully built 1dedfc47bf1d
[00:14:47] Successfully tagged rust-ci:latest
[00:14:47] Built container sha256:1dedfc47bf1d028f492440130587fe5eff50239e827b157b4a40c58c735a2e75
[00:14:47] Uploading finished image to s3://rust-lang-ci-sccache2/docker/32f4ccd5a546fa682918b37328422c50702c59796142f6b87c1c2fbc3f6bddca51b48622a9b67371b288dec8a175746152837aebc638ce07a76417a8db6c0b68
[00:20:01] upload failed: - to s3://rust-lang-ci-sccache2/docker/32f4ccd5a546fa682918b37328422c50702c59796142f6b87c1c2fbc3f6bddca51b48622a9b67371b288dec8a175746152837aebc638ce07a76417a8db6c0b68 Unable to locate credentials

[00:20:02] travis_time:end:056bb060:start=1544143103248326146,finish=1544144292643153811,duration=1189394827665
[CI_JOB_NAME=dist-x86_64-musl]
[00:20:02] [CI_JOB_NAME=dist-x86_64-musl]
---
142168 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
134884 ./.git/modules
134880 ./.git/modules/src
134772 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134768 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f7cq469z9z-yp81ro-bfczq5obtrde
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
95108 ./src/tools/clang/test
89968 ./src/llvm-emscripten/test/CodeGen
---
56832 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
56828 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
54028 ./.git/modules/src/tools
52864 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl
52860 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0336de00
travis_time:start:0336de00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c856cf1
$ dmesg | grep -i kill
