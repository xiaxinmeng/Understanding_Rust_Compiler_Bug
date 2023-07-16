plain
travis_time:end:1a470f26:start=1554457028609868861,finish=1554457136322246540,duration=107712377679
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
[01:19:29] 
[01:19:29] running 9 tests
[01:19:29] iiiiiiiii
[01:19:29] 
[01:19:29]  finished in 0.164
[01:19:29] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:47] 
[01:19:47] running 121 tests
[01:20:17] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:23] i.i......iii.i.....ii
[01:20:23] 
[01:20:23]  finished in 35.531
[01:20:23] travis_fold:end:test_debuginfo

---
[01:32:15] ...............................................................................i.i.................. 400/931
[01:32:15] .................................................................................................... 500/931
[01:32:15] .................................................................................................... 600/931
[01:32:15] .................................................................................................... 700/931
[01:32:15] ......................F........................................F.................................... 800/931
[01:32:17] ...............................
[01:32:17] failures:
[01:32:17] 
[01:32:17] ---- option::test_collect stdout ----
---
[01:32:17] 
[01:32:17] error: test failed, to rerun pass '--test coretests'
[01:32:17] 
[01:32:17] 
[01:32:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:32:17] 
[01:32:17] 
[01:32:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:17] Build completed unsuccessfully in 0:25:57
[01:32:17] Build completed unsuccessfully in 0:25:57
[01:32:17] Makefile:48: recipe for target 'check' failed
[01:32:17] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b08d661
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 11:11:25 UTC 2019
---
travis_time:end:11e8dcf2:start=1554462687366630339,finish=1554462687372440308,duration=5809969
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:076c7964
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11870736
travis_time:start:11870736
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1968b032
$ dmesg | grep -i kill
