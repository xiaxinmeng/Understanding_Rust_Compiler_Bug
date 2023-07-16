plain
travis_time:end:06ce6aab:start=1557841459953108408,finish=1557841460826629759,duration=873521351
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:05:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:11] tidy error: /checkout/src/etc/gdb_providers.py:14: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/gdb_providers.py:154: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:6: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:17: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:35: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:62: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:350: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:435: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:478: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/etc/lldb_providers.py:479: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1012: line longer than 100 chars
[00:05:11] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1013: line longer than 100 chars
[00:05:16] some tidy checks failed
[00:05:16] 
[00:05:16] 
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:16] 
[00:05:16] 
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:16] Build completed unsuccessfully in 0:01:09
[00:05:16] Build completed unsuccessfully in 0:01:09
[00:05:16] Makefile:67: recipe for target 'tidy' failed
[00:05:16] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06811542
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 13:49:47 UTC 2019
---
travis_time:end:0d1b9ce0:start=1557841788424075907,finish=1557841788428436476,duration=4360569
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a81a2f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:069a6ac8
travis_time:start:069a6ac8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22dd5b59
$ dmesg | grep -i kill
