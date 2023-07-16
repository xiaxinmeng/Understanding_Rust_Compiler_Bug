plain
travis_time:end:1583ee00:start=1541493552633335169,finish=1541493628668214801,duration=76034879632
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:21] 
[00:04:21] To learn more, run the command again with --verbose.
[00:04:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:21] expected success, got: exit code: 101
[00:04:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:04:21] travis_fold:end:stage0-std

[00:04:21] travis_time:end:stage0-std:start=1541493879326450331,finish=1541493900183991471,duration=20857541140


[00:04:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:21] Build completed unsuccessfully in 0:00:22
[00:04:21] make: *** [all] Error 1
[00:04:21] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:179eefaf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03955f36:start=1541493900866188328,finish=1541493900871916833,duration=5728505
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16e3f5a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:072d9118
travis_time:start:072d9118
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1788d25b
$ dmesg | grep -i kill
