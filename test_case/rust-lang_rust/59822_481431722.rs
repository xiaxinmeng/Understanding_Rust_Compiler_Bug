plain
travis_time:end:033b22db:start=1554836944518034636,finish=1554837030288527768,duration=85770493132
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
[01:13:33] 
[01:13:33] running 9 tests
[01:13:33] iiiiiiiii
[01:13:33] 
[01:13:33]  finished in 0.156
[01:13:33] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:50] 
[01:13:50] running 121 tests
[01:14:18] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:23] i.i......iii.i.....ii
[01:14:23] 
[01:14:23]  finished in 33.779
[01:14:23] travis_fold:end:test_debuginfo

---
[01:43:44] travis_time:end:stage0-rustdoc-themes:start=1554843262683959351,finish=1554843263693558282,duration=1009598931

[01:43:44] rustdoc: [theme-checker] Starting tests!
[01:43:44]  - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... FAILED
[01:43:44]   Missing ".stab > code" rule
[01:43:44] 
[01:43:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
[01:43:44] expected success, got: exit code: 1
[01:43:44] 
[01:43:44] 
[01:43:44] 
[01:43:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:44] Build completed unsuccessfully in 0:42:42
[01:43:44] Makefile:48: recipe for target 'check' failed
[01:43:44] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1513f6cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  9 20:54:24 UTC 2019
---
travis_time:end:091bab24:start=1554843266463418512,finish=1554843266468398546,duration=4980034
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:125f3026
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:047cbe60
travis_time:start:047cbe60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14e3ad6e
$ dmesg | grep -i kill
