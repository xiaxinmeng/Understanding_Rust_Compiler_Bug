plain
travis_time:end:1382f5f8:start=1546526062986574057,finish=1546526064103103118,duration=1116529061
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
tidy check
[00:03:30] * 568 error codes
[00:03:30] * highest error code: E0721
[00:03:31] * 245 features
[00:03:31] invalid source: "git+https://github.com/rust-lang/rust-clippy?rev=a3c77f6ad1c1c185e561e9cd7fdec7db569169d1#a3c77f6ad1c1c185e561e9cd7fdec7db569169d1"
[00:03:31] some tidy checks failed
[00:03:31] 
[00:03:31] 
[00:03:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:31] 
[00:03:31] 
[00:03:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:31] Build completed unsuccessfully in 0:00:45
[00:03:31] Build completed unsuccessfully in 0:00:45
[00:03:31] Makefile:69: recipe for target 'tidy' failed
[00:03:31] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cb5dc30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 14:38:05 UTC 2019
---
travis_time:end:12986fc4:start=1546526285488969273,finish=1546526285494029756,duration=5060483
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c0f7c68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ecc4029
travis_time:start:1ecc4029
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02226ca2
$ dmesg | grep -i kill
