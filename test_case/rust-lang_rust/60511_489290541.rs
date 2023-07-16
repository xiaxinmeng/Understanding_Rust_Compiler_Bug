plain
travis_time:end:14e7e8c1:start=1556934960143166653,finish=1556935048138362582,duration=87995195929
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
[01:23:46] 
[01:23:46] running 9 tests
[01:23:46] iiiiiiiii
[01:23:46] 
[01:23:46]  finished in 0.178
[01:23:46] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:03] 
[01:24:03] running 121 tests
[01:24:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:24:34] i.i......iii.i.....ii
[01:24:34] 
[01:24:34]  finished in 31.284
[01:24:34] travis_fold:end:test_debuginfo

---
[01:47:05] travis_fold:end:stage0-linkchecker

[01:47:05] travis_time:end:stage0-linkchecker:start=1556941480650586094,finish=1556941482587325810,duration=1936739716

[01:47:09] std/collections/hash_map/struct.DefaultHasher.html:25: broken link - std/std/collections/hash_map/struct.DefaultHasher.html
[01:47:13] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:47:13] 
[01:47:13] 
[01:47:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:47:13] expected success, got: exit code: 101
[01:47:13] expected success, got: exit code: 101
[01:47:13] 
[01:47:13] 
[01:47:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:13] Build completed unsuccessfully in 0:36:13
[01:47:13] Makefile:48: recipe for target 'check' failed
[01:47:13] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007b8c91
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May  4 03:44:51 UTC 2019
---
travis_time:end:0f8ae790:start=1556941492882380613,finish=1556941492900939342,duration=18558729
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:073f54c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f69abde
travis_time:start:0f69abde
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06ceda32
$ dmesg | grep -i kill
