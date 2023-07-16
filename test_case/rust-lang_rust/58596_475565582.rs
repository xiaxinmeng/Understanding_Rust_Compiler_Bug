plain
travis_time:end:0d2a15b3:start=1553243607992122098,finish=1553243608881277650,duration=889155552
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
[01:18:39] 
[01:18:39] running 9 tests
[01:18:39] iiiiiiiii
[01:18:39] 
[01:18:39]  finished in 0.146
[01:18:39] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:54] 
[01:18:54] running 120 tests
[01:19:18] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:19:23] .i......iii.i.....ii
[01:19:23] 
[01:19:23]  finished in 28.501
[01:19:23] travis_fold:end:test_debuginfo

---
[01:44:19] travis_fold:end:stage0-linkchecker

[01:44:19] travis_time:end:stage0-linkchecker:start=1553249876514449711,finish=1553249878615474062,duration=2101024351

[01:44:21] std/primitive.str.html:1142: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:44:21] std/primitive.str.html:1165: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:44:21] std/primitive.str.html:1186: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:44:26] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:44:26] 
[01:44:26] 
[01:44:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:44:26] expected success, got: exit code: 101
[01:44:26] expected success, got: exit code: 101
[01:44:26] 
[01:44:26] 
[01:44:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:26] Build completed unsuccessfully in 0:36:59
[01:44:26] Makefile:48: recipe for target 'check' failed
[01:44:26] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04d084c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 22 10:18:06 UTC 2019
---
travis_time:end:0bdddae1:start=1553249888321494669,finish=1553249888326445552,duration=4950883
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1da607b1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0033780e
travis_time:start:0033780e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:020f99d9
$ dmesg | grep -i kill
