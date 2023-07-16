plain
travis_time:end:1fec1371:start=1551327734658199878,finish=1551327830255679164,duration=95597479286
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
[01:17:36] 
[01:17:36] running 120 tests
[01:18:05] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:18:10] .i......iii.i.....ii
[01:18:10] 
[01:18:10]  finished in 33.834
[01:18:10] travis_fold:end:test_debuginfo

---
[01:29:12] 
[01:29:12]    Doc-tests core
[01:29:18] 
[01:29:18] running 2277 tests
[01:29:31] ....iiiii........................................................................................... 100/2277
[01:29:45] .....................................................................ii.....F....................... 200/2277
[01:30:18] .................................................................................................... 400/2277
[01:30:33] .....................i..i........................................................................... 500/2277
[01:30:47] .................................................................................................... 600/2277
[01:31:01] .................................................................................................... 700/2277
---
[01:35:04] 
[01:35:04] error: test failed, to rerun pass '--doc'
[01:35:04] 
[01:35:04] 
[01:35:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:35:04] 
[01:35:04] 
[01:35:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:04] Build completed unsuccessfully in 0:30:17
[01:35:04] Build completed unsuccessfully in 0:30:17
[01:35:04] Makefile:48: recipe for target 'check' failed
[01:35:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a4c4981
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 05:59:04 UTC 2019
---
travis_time:end:03406980:start=1551333546545784497,finish=1551333546551723911,duration=5939414
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cad00a3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03194e53
travis_time:start:03194e53
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0af78c93
$ dmesg | grep -i kill
