plain
travis_time:end:17d39c66:start=1544383883260207989,finish=1544383884276087416,duration=1015879427
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
tidy check
[00:02:59] * 568 error codes
[00:02:59] * highest error code: E0721
[00:02:59] * 242 features
[00:03:00] crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
[00:03:00]   * rustc-ap-syntax 297.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:03:00]   * rustc-ap-syntax 306.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:03:00] some tidy checks failed
[00:03:00] 
[00:03:00] 
[00:03:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:00] 
[00:03:00] 
[00:03:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:00] Build completed unsuccessfully in 0:00:53
[00:03:00] Build completed unsuccessfully in 0:00:53
[00:03:00] make: *** [tidy] Error 1
[00:03:00] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:068fea94
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  9 19:34:32 UTC 2018
---
travis_time:end:16b2325e:start=1544384073251737399,finish=1544384073257134210,duration=5396811
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f93ce0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22cb4bcb
travis_time:start:22cb4bcb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02891334
$ dmesg | grep -i kill
