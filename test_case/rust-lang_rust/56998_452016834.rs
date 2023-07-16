plain
travis_time:end:09b782d5:start=1546882310539222743,finish=1546882411740803713,duration=101201580970
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:tidy
tidy check
[00:03:27] * 569 error codes
[00:03:27] * highest error code: E0721
[00:03:27] tidy error: /checkout/src/liballoc/boxed.rs:542: malformed stability attribute
[00:03:28] some tidy checks failed
[00:03:28] 
[00:03:28] 
[00:03:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:28] 
[00:03:28] 
[00:03:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:28] Build completed unsuccessfully in 0:00:46
[00:03:28] Build completed unsuccessfully in 0:00:46
[00:03:28] make: *** [tidy] Error 1
[00:03:28] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0477af3b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan  7 17:37:08 UTC 2019
---
travis_time:end:237f02cb:start=1546882629339011605,finish=1546882629344753950,duration=5742345
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16531626
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f6b9666
travis_time:start:0f6b9666
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02bb55fe
$ dmesg | grep -i kill
