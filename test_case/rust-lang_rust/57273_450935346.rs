plain
travis_time:end:189adcf2:start=1546451752158214839,finish=1546451753179337172,duration=1021122333
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:59]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:04]    Compiling compiler_builtins v0.1.2
[00:03:04]    Compiling cmake v0.1.33
[00:03:04]    Compiling backtrace-sys v0.1.27
[00:03:05] error: the feature named `cmpxchg16b` is not valid for this target
[00:03:05]   --> src/libcore/../stdsimd/coresimd/x86_64/cmpxchg16b.rs:44:18
[00:03:05]    |
[00:03:05] 44 | #[target_feature(enable = "cmpxchg16b")]
[00:03:05] 
[00:03:07]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:07]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:07]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
---
[00:03:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:15] expected success, got: exit code: 101
[00:03:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:15] Build completed unsuccessfully in 0:00:19
[00:03:15] make: *** [all] Error 1
[00:03:15] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19db7649
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan  2 17:59:21 UTC 2019
---
travis_time:end:21ec0891:start=1546451962008106403,finish=1546451962012686248,duration=4579845
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:279fd716
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:099e92cc
travis_time:start:099e92cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1688251b
$ dmesg | grep -i kill
