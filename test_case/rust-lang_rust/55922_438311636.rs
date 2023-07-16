plain
travis_time:end:037b8eb4:start=1542122792656641651,finish=1542122849251124918,duration=56594483267
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:12:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:4: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:5: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:6: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:7: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:8: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:9: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs:10: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const.rs: missing trailing newline
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:4: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:5: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:6: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:7: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:8: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:9: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs:10: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-3.rs: missing trailing newline
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:4: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:5: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:6: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:7: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:8: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:9: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs:10: tab character
[00:12:26] tidy error: /checkout/src/test/ui/pattern/slice-pattern-const-2.rs: missing trailing newline
[00:12:27] some tidy checks failed
[00:12:27] 
[00:12:27] 
[00:12:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:12:27] 
[00:12:27] 
[00:12:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:12:27] Build completed unsuccessfully in 0:00:49
[00:12:27] Build completed unsuccessfully in 0:00:49
[00:12:27] Makefile:79: recipe for target 'tidy' failed
[00:12:27] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:135bec9d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 15:40:07 UTC 2018
---
travis_time:end:0b42748e:start=1542123607937563409,finish=1542123607943313751,duration=5750342
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25a8f066
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07f182a4
travis_time:start:07f182a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28fac8f0
$ dmesg | grep -i kill
