plain
travis_time:end:06ec07b4:start=1545082422890926254,finish=1545082426621918026,duration=3730991772
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:30]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:30]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:31]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:31]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:33] error[E0441]: unrecognized platform-specific intrinsic function: `simd_select_bitmask`
[00:03:33]   --> src/libcore/../stdsimd/coresimd/simd_llvm.rs:52:5
[00:03:33]    |
[00:03:33] 52 |     pub fn simd_select_bitmask<M, T>(m: M, a: T, b: T) -> T;
[00:03:33] 
[00:03:33] error: aborting due to previous error
[00:03:33] 
[00:03:33] For more information about this error, try `rustc --explain E0441`.
[00:03:33] For more information about this error, try `rustc --explain E0441`.
[00:03:33] error: Could not compile `core`.
[00:03:33] warning: build failed, waiting for other jobs to finish...
[00:03:35] error: build failed
[00:03:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:35] expected success, got: exit code: 101
[00:03:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:35] Build completed unsuccessfully in 0:00:16
[00:03:35] Makefile:28: recipe for target 'all' failed
[00:03:35] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0de7b663
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 21:37:33 UTC 2018
---
travis_time:end:040a7ee3:start=1545082653650558300,finish=1545082653657355096,duration=6796796
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:141334f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0394e3c0
travis_time:start:0394e3c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04e44a5b
$ dmesg | grep -i kill
