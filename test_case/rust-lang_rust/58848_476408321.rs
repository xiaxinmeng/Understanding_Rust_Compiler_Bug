plain
travis_time:end:1b72875c:start=1553547769377239303,finish=1553547868635908936,duration=99258669633
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:58] 
[01:21:58] running 9 tests
[01:21:58] iiiiiiiii
[01:21:58] 
[01:21:58]  finished in 0.163
[01:21:58] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:15] 
[01:22:15] running 120 tests
[01:22:43] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:48] .i......iii.i.....ii
[01:22:48] 
[01:22:48]  finished in 33.724
[01:22:48] travis_fold:end:test_debuginfo

---
[01:49:19] travis_fold:end:stage0-linkchecker

[01:49:19] travis_time:end:stage0-linkchecker:start=1553554435560428855,finish=1553554437808548858,duration=2248120003

[01:49:31] error-index.html:7: broken link - light.css
[01:49:31] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:49:31] 
[01:49:31] 
[01:49:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:49:31] expected success, got: exit code: 101
[01:49:31] expected success, got: exit code: 101
[01:49:31] 
[01:49:31] 
[01:49:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:31] Build completed unsuccessfully in 0:40:11
[01:49:31] make: *** [check] Error 1
[01:49:31] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:114df461
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 22:54:09 UTC 2019
---
travis_time:end:01421079:start=1553554452314884963,finish=1553554452320145188,duration=5260225
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cdf8e94
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004590ea
travis_time:start:004590ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24edb5d0
$ dmesg | grep -i kill
