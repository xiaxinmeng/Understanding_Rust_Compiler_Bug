plain
travis_time:end:3393d272:start=1547328588020696191,finish=1547328669354442686,duration=81333746495
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:47] 
[01:12:47] running 118 tests
[01:13:12] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:13:17] ......iii.i.....ii
[01:13:17] 
[01:13:17]  finished in 29.641
[01:13:17] travis_fold:end:test_debuginfo

---
[01:41:04] travis_fold:end:stage0-linkchecker

[01:41:04] travis_time:end:stage0-linkchecker:start=1547334740123542332,finish=1547334742173308995,duration=2049766663

[01:41:06] settings.html:1: broken link - favicon.ico
[01:41:12] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:39:9
[01:41:12] 
[01:41:12] 
[01:41:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:41:12] expected success, got: exit code: 101
[01:41:12] expected success, got: exit code: 101
[01:41:12] 
[01:41:12] 
[01:41:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:12] Build completed unsuccessfully in 0:40:12
[01:41:12] make: *** [check] Error 1
[01:41:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16356450
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 23:12:30 UTC 2019
---
travis_time:end:03a49628:start=1547334751833000743,finish=1547334751838704482,duration=5703739
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01778b1a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03ad72c0
travis_time:start:03ad72c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03338160
$ dmesg | grep -i kill
