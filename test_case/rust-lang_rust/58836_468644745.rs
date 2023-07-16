plain
travis_time:end:06be49a3:start=1551436312113186340,finish=1551436314706104526,duration=2592918186
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
[01:14:42] 
[01:14:42] running 119 tests
[01:15:07] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:11] i......iii.i.....ii
[01:15:11] 
[01:15:11]  finished in 29.572
[01:15:11] travis_fold:end:test_debuginfo

---
[01:36:14] 
[01:36:14] To learn more, run the command again with --verbose.
[01:36:14] 
[01:36:14] 
[01:36:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:36:14] 
[01:36:14] 
[01:36:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:14] Build completed unsuccessfully in 0:33:12
[01:36:14] Build completed unsuccessfully in 0:33:12
[01:36:14] Makefile:48: recipe for target 'check' failed
[01:36:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a479dd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 12:08:18 UTC 2019
---
travis_time:end:0f1d7211:start=1551442100434435312,finish=1551442100439075918,duration=4640606
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ca911c5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00437b02
travis_time:start:00437b02
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b05b536
$ dmesg | grep -i kill
