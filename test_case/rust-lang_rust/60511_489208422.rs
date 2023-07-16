plain
travis_time:end:2c2993af:start=1556904600153122637,finish=1556904602746586546,duration=2593463909
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:49] 
[01:18:49] running 9 tests
[01:18:49] iiiiiiiii
[01:18:49] 
[01:18:49]  finished in 0.152
[01:18:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:05] 
[01:19:05] running 121 tests
[01:19:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:19:35] i.i......iii.i.....ii
[01:19:35] 
[01:19:35]  finished in 30.379
[01:19:35] travis_fold:end:test_debuginfo

---
[01:42:06] travis_fold:end:stage0-linkchecker

[01:42:06] travis_time:end:stage0-linkchecker:start=1556910736254725633,finish=1556910738403086413,duration=2148360780

[01:42:10] std/default/trait.Default.html:100: broken link - collections/hash_map/struct.DefaultHasher.html
[01:42:14] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:42:14] 
[01:42:14] 
[01:42:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:42:14] expected success, got: exit code: 101
[01:42:14] expected success, got: exit code: 101
[01:42:14] 
[01:42:14] 
[01:42:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:14] Build completed unsuccessfully in 0:35:04
[01:42:14] Makefile:48: recipe for target 'check' failed
[01:42:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0772cab2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 19:12:27 UTC 2019
---
travis_time:end:2e25702d:start=1556910749050609395,finish=1556910749057102353,duration=6492958
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08776908
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02268ac0
travis_time:start:02268ac0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08ec0d1a
$ dmesg | grep -i kill
