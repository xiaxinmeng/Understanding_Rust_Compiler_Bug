plain
travis_time:end:144d3956:start=1548855124528382813,finish=1548855126506788706,duration=1978405893
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:16]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:17]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:17]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:18]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:33] error[E0635]: unknown feature `movbe_target_feature`
[00:04:33]     |
[00:04:33]     |
[00:04:33] 126 | #![feature(movbe_target_feature)]
[00:04:33] 
[00:04:33] error: aborting due to previous error
[00:04:33] 
[00:04:33] For more information about this error, try `rustc --explain E0635`.
[00:04:33] For more information about this error, try `rustc --explain E0635`.
[00:04:33] error: Could not compile `core`.
[00:04:33] 
[00:04:33] To learn more, run the command again with --verbose.
[00:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:33] expected success, got: exit code: 101
[00:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:33] Build completed unsuccessfully in 0:00:29
[00:04:33] Makefile:18: recipe for target 'all' failed
[00:04:33] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04833bab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 13:36:55 UTC 2019
---
travis_time:end:06f5d438:start=1548855416353309282,finish=1548855416358356646,duration=5047364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04647816
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0882d5f4
travis_time:start:0882d5f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0823e10c
$ dmesg | grep -i kill
