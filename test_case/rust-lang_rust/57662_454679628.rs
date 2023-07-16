plain
travis_time:end:2db3b706:start=1547623480279563046,finish=1547623481241714349,duration=962151303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:29] tidy error: /checkout/src/libsyntax/ast.rs:2092: line longer than 100 chars
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:42: trailing whitespace
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:500: line longer than 100 chars
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:738: line longer than 100 chars
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:752: line longer than 100 chars
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:758: line longer than 100 chars
[00:03:29] tidy error: /checkout/src/libsyntax/visit_mut.rs:764: line longer than 100 chars
[00:03:31] some tidy checks failed
[00:03:31] 
[00:03:31] 
[00:03:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:31] 
[00:03:31] 
[00:03:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:31] Build completed unsuccessfully in 0:00:46
[00:03:31] Build completed unsuccessfully in 0:00:46
[00:03:31] make: *** [tidy] Error 1
[00:03:31] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ae948f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 07:28:23 UTC 2019
---
travis_time:end:0a9b0cc2:start=1547623704375112740,finish=1547623704379824881,duration=4712141
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06d18916
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:071e2786
travis_time:start:071e2786
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b1fdee1
$ dmesg | grep -i kill
