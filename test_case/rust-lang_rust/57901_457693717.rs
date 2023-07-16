plain
travis_time:end:10d9727c:start=1548442033018742450,finish=1548442035633104723,duration=2614362273
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:34] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:34] tidy error: binary checked into source: /checkout/src/test/ui/issues/issue-57362.rs
[00:04:35] tidy error: /checkout/src/test/ui/issues/issue-57362.rs: missing trailing newline
[00:04:36] some tidy checks failed
[00:04:36] 
[00:04:36] 
[00:04:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:36] 
[00:04:36] 
[00:04:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:36] Build completed unsuccessfully in 0:00:46
[00:04:36] Build completed unsuccessfully in 0:00:46
[00:04:36] Makefile:68: recipe for target 'tidy' failed
[00:04:36] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12976398
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 25 18:52:03 UTC 2019
---
travis_time:end:06cef247:start=1548442323575264000,finish=1548442323579840139,duration=4576139
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a69b642
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0142bd95
travis_time:start:0142bd95
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12ab855c
$ dmesg | grep -i kill
