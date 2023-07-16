plain
travis_time:end:01095d63:start=1550919481497837316,finish=1550919578432629687,duration=96934792371
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:18] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:19] tidy error: /checkout/src/librustc_codegen_ssa/mir/mod.rs:629: line longer than 100 chars
[00:03:20] some tidy checks failed
[00:03:20] 
[00:03:20] 
[00:03:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:20] 
[00:03:20] 
[00:03:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:20] Build completed unsuccessfully in 0:00:47
[00:03:20] Build completed unsuccessfully in 0:00:47
[00:03:20] make: *** [tidy] Error 1
[00:03:20] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b505f33
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 23 11:03:09 UTC 2019
---
travis_time:end:256d7549:start=1550919790032538963,finish=1550919790037634066,duration=5095103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02a0a1d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10ecfcb7
travis_time:start:10ecfcb7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02033d00
$ dmesg | grep -i kill
