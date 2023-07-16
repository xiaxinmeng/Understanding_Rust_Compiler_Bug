plain
travis_time:end:06ae2b3e:start=1550008356428108442,finish=1550009002085614826,duration=645657506384
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
[01:10:41] 
[01:10:41] running 119 tests
[01:11:06] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:11] i......iii.i.....ii
[01:11:11] 
[01:11:11]  finished in 29.961
[01:11:11] travis_fold:end:test_debuginfo

---
[01:30:17] .................................................................................................... 600/988
[01:30:25] .................................................................................................... 700/988
[01:30:34] .........iiii....................................................................................... 800/988
[01:30:46] .................................................................................................... 900/988
[01:30:53] ...................................iiii...................................F.............
[01:30:53] 
[01:30:53] ---- time.rs - time::Instant::checked_duration_since (line 226) stdout ----
[01:30:53] ---- time.rs - time::Instant::checked_duration_since (line 226) stdout ----
[01:30:53] error[E0658]: use of unstable library feature 'checked_duration_since' (see issue #58402)
[01:30:53]   --> time.rs:233:26
[01:30:53]    |
[01:30:53] 10 | println!("{:?}", new_now.checked_duration_since(now));
[01:30:53]    |
[01:30:53]    |
[01:30:53]    = help: add #![feature(checked_duration_since)] to the crate attributes to enable
[01:30:53] 
[01:30:53] error[E0658]: use of unstable library feature 'checked_duration_since' (see issue #58402)
[01:30:53]   --> time.rs:234:22
[01:30:53]    |
[01:30:53] 11 | println!("{:?}", now.checked_duration_since(new_now)); // None
[01:30:53]    |
[01:30:53]    |
[01:30:53]    = help: add #![feature(checked_duration_since)] to the crate attributes to enable
[01:30:53] thread 'time.rs - time::Instant::checked_duration_since (line 226)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:30:53] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:30:53] 
[01:30:53] 
---
[01:30:53] 
[01:30:53] error: test failed, to rerun pass '--doc'
[01:30:53] 
[01:30:53] 
[01:30:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:30:53] 
[01:30:53] 
[01:30:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:53] Build completed unsuccessfully in 0:31:55
[01:30:53] Build completed unsuccessfully in 0:31:55
[01:30:53] make: *** [check] Error 1
[01:30:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e3222fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 23:34:24 UTC 2019
---
travis_time:end:0df0d5e7:start=1550014466395662680,finish=1550014466401999081,duration=6336401
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:126c2048
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:132c4a9a
travis_time:start:132c4a9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:083d6ad7
$ dmesg | grep -i kill
