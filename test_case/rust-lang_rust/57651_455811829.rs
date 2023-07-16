plain
travis_time:end:014e67f1:start=1547926772624169260,finish=1547926844755820702,duration=72131651442
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:24] tidy error: /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs: missing trailing newline
[00:03:25] some tidy checks failed
[00:03:25] 
[00:03:25] 
[00:03:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:25] 
[00:03:25] 
[00:03:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:25] Build completed unsuccessfully in 0:00:46
[00:03:25] Build completed unsuccessfully in 0:00:46
[00:03:25] make: *** [tidy] Error 1
[00:03:25] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a21e539
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 19:44:20 UTC 2019
---
travis_time:end:1a5e722c:start=1547927060717380708,finish=1547927060722566739,duration=5186031
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0167d750
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:264c1ce6
travis_time:start:264c1ce6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17a44301
$ dmesg | grep -i kill
