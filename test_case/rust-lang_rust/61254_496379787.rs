plain
travis_time:end:02790c9d:start=1559023239613158030,finish=1559023240438911518,duration=825753488
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:33] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:33] tidy error: /checkout/src/libsyntax/parse/diagnostics.rs: missing trailing newline
[00:04:34] tidy error: /checkout/src/test/ui/suggestions/suggest-move-types.rs: ignoring line length unnecessarily
[00:04:38] some tidy checks failed
[00:04:38] 
[00:04:38] 
[00:04:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:38] 
[00:04:38] 
[00:04:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:38] Build completed unsuccessfully in 0:01:12
[00:04:38] Build completed unsuccessfully in 0:01:12
[00:04:38] make: *** [tidy] Error 1
[00:04:38] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036762b5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 06:05:30 UTC 2019
---
travis_time:end:258938a0:start=1559023530939598365,finish=1559023530944543368,duration=4945003
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15106170
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13c256e8
travis_time:start:13c256e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03d15a94
$ dmesg | grep -i kill
