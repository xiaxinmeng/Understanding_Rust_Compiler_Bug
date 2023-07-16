plain
travis_time:end:00765b41:start=1556790413407114313,finish=1556790499631178683,duration=86224064370
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:30] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:30] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:1189: line longer than 100 chars
[00:03:32] some tidy checks failed
[00:03:32] 
[00:03:32] 
[00:03:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:32] 
[00:03:32] 
[00:03:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:32] Build completed unsuccessfully in 0:00:44
[00:03:32] Build completed unsuccessfully in 0:00:44
[00:03:32] make: *** [tidy] Error 1
[00:03:32] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09bdbf07
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 09:52:01 UTC 2019
---
travis_time:end:0e8f5354:start=1556790722008773624,finish=1556790722013525122,duration=4751498
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a0f4a78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:023350ec
travis_time:start:023350ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022d8b40
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[    0.753148] misc rfkill: hash matches
travis_fold:end:after_failure.6

Done. Your build exited with 1.
