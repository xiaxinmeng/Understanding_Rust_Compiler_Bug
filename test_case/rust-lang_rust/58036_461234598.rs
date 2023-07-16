plain
travis_time:end:1e51b6a9:start=1549492312199195806,finish=1549492318484609248,duration=6285413442
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
[01:07:44] 
[01:07:44] running 119 tests
[01:08:08] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:08:12] i......iii.i.....ii
[01:08:12] 
[01:08:12]  finished in 28.082
[01:08:12] travis_fold:end:test_debuginfo

---
[01:17:12] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:12]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:17:13] error: expected one of `!` or `::`, found `std`
[01:17:13]   |
[01:17:13]   |
[01:17:13] 1 | cuse std::str::pattern::*;
[01:17:13]   |      ^^^ expected one of `!` or `::` here
[01:17:13] error: aborting due to previous error
[01:17:13] 
[01:17:13] error: Could not compile `core`.
[01:17:13] 
[01:17:13] 
[01:17:13] To learn more, run the command again with --verbose.
[01:17:13] 
[01:17:13] 
[01:17:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:17:13] 
[01:17:13] 
[01:17:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:13] Build completed unsuccessfully in 0:20:41
[01:17:13] Build completed unsuccessfully in 0:20:41
[01:17:13] Makefile:48: recipe for target 'check' failed
[01:17:13] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e02622a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 23:49:21 UTC 2019
---
travis_time:end:0d6926fc:start=1549496962748483836,finish=1549496962753083939,duration=4600103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b19630e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:191dc135
travis_time:start:191dc135
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09ed3c38
$ dmesg | grep -i kill
