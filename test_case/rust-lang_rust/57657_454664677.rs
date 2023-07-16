plain
travis_time:end:010122b7:start=1547617107432364229,finish=1547617178436539574,duration=71004175345
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:36] tidy error: /checkout/src/test/ui/issue-53787-inline-assembler-macro.rs:8: trailing whitespace
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:45
[00:03:37] Build completed unsuccessfully in 0:00:45
[00:03:37] Makefile:69: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d535a80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 05:43:25 UTC 2019
---
travis_time:end:075edd5d:start=1547617405877264294,finish=1547617405881800427,duration=4536133
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0892ee58
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01f7467e
travis_time:start:01f7467e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:061adb8e
$ dmesg | grep -i kill
