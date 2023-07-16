plain
travis_time:end:1282d748:start=1547302837816421733,finish=1547302916289343118,duration=78472921385
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
[00:03:24] tidy error: /checkout/src/librustc_errors/diagnostic.rs:157: line longer than 100 chars
[00:03:25] tidy error: /checkout/src/librustc/ty/wf.rs:384: trailing whitespace
[00:03:25] tidy error: /checkout/src/test/ui/wf/wf-convert-unsafe-trait-obj-box.rs:1: trailing whitespace
[00:03:25] tidy error: /checkout/src/test/ui/wf/wf-convert-unsafe-trait-obj.rs:1: trailing whitespace
[00:03:25] tidy error: /checkout/src/test/ui/rfc-2027-object-safe-for-dispatch/static-dispatch-unsafe-object.rs:29: trailing whitespace
[00:03:26] some tidy checks failed
[00:03:26] 
[00:03:26] 
[00:03:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:26] 
[00:03:26] 
[00:03:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:26] Build completed unsuccessfully in 0:00:49
[00:03:26] Build completed unsuccessfully in 0:00:49
[00:03:26] Makefile:69: recipe for target 'tidy' failed
[00:03:26] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e33db06
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 14:25:33 UTC 2019
---
travis_time:end:03206151:start=1547303134021835351,finish=1547303134027035184,duration=5199833
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007dadb1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05200d64
travis_time:start:05200d64
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14069ac0
$ dmesg | grep -i kill
