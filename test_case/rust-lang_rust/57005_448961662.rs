plain
travis_time:end:02e2c77c:start=1545304257291336329,finish=1545304314094450253,duration=56803113924
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:02]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:02]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:02]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:02]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:04] error[E0441]: unrecognized platform-specific intrinsic function: `simd_select_bitmask`
[00:03:04]   --> src/libcore/../stdsimd/coresimd/simd_llvm.rs:52:5
[00:03:04]    |
[00:03:04] 52 |     pub fn simd_select_bitmask<M, T>(m: M, a: T, b: T) -> T;
[00:03:04] 
[00:03:04] error: aborting due to previous error
[00:03:04] 
[00:03:04] For more information about this error, try `rustc --explain E0441`.
[00:03:04] For more information about this error, try `rustc --explain E0441`.
[00:03:04] error: Could not compile `core`.
[00:03:04] warning: build failed, waiting for other jobs to finish...
[00:03:06] error: build failed
[00:03:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:06] expected success, got: exit code: 101
[00:03:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:06] Build completed unsuccessfully in 0:00:15
[00:03:06] Makefile:28: recipe for target 'all' failed
[00:03:06] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1445c043
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 20 11:15:09 UTC 2018
---
travis_time:end:050311c2:start=1545304509749277236,finish=1545304509754266239,duration=4989003
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:087f0324
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03297ea0
travis_time:start:03297ea0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0437a66e
$ dmesg | grep -i kill
