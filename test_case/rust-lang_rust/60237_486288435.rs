plain
travis_time:end:16ea328d:start=1556118243011137315,finish=1556118367342870921,duration=124331733606
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:45] tidy error: /checkout/src/librustc_target/abi/mod.rs:961: line longer than 100 chars
[00:03:45] tidy error: /checkout/src/librustc_target/abi/mod.rs:961: trailing whitespace
[00:03:47] some tidy checks failed
[00:03:47] 
[00:03:47] 
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:47] 
[00:03:47] 
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:00:42
[00:03:47] Build completed unsuccessfully in 0:00:42
[00:03:47] make: *** [tidy] Error 1
[00:03:47] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0782dccc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 15:10:04 UTC 2019
---
travis_time:end:0147239e:start=1556118605034713655,finish=1556118605039181324,duration=4467669
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21d25584
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02aa37fd
travis_time:start:02aa37fd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0754baa6
$ dmesg | grep -i kill
