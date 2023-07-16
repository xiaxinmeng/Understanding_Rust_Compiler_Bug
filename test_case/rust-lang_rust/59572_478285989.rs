plain
travis_time:end:069d02d8:start=1553976020371508047,finish=1553976096154014026,duration=75782505979
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:31] tidy error: /checkout/src/librustc_passes/ast_validation.rs:357: line longer than 100 chars
[00:04:32] some tidy checks failed
[00:04:32] 
[00:04:32] 
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] 
[00:04:32] 
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:00:47
[00:04:32] Build completed unsuccessfully in 0:00:47
[00:04:32] Makefile:67: recipe for target 'tidy' failed
[00:04:32] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f0eece
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 30 20:06:18 UTC 2019
---
travis_time:end:106a8e33:start=1553976378968155704,finish=1553976378973239856,duration=5084152
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f37eee7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14b97fd0
travis_time:start:14b97fd0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08a8eede
$ dmesg | grep -i kill
