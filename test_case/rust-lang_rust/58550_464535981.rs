plain
travis_time:end:0b7449c8:start=1550444884109648215,finish=1550444962037057684,duration=77927409469
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
[01:12:28] 
[01:12:28] running 119 tests
[01:12:54] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:58] i......iii.i.....ii
[01:12:58] 
[01:12:58]  finished in 30.479
[01:12:58] travis_fold:end:test_debuginfo

---
[01:31:40] ..........iii......i......i...i......i.............................................................. 300/992
[01:31:45] .................................................................................................... 400/992
[01:31:54] ........................i.i.....................................iiii........ii...................... 500/992
[01:32:01] .................................................................................................... 600/992
[01:32:09] ............................................................................................F....... 700/992
[01:32:31] .................................................................................................... 900/992
[01:32:38] ......................................iiii..................................................
[01:32:38] failures:
[01:32:38] 
[01:32:38] 
[01:32:38] ---- path.rs - path::PathBuf::with_capacity (line 1153) stdout ----
[01:32:38] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:38]  --> path.rs:1156:12
[01:32:38] 6 | let path = PathBuf::with_capacity(10);
[01:32:38]   |            ^^^^^^^^^^^^^^^^^^^^^^
[01:32:38]   |
[01:32:38]   |
[01:32:38]   = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:38] 
[01:32:38] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:38]  --> path.rs:1157:21
[01:32:38]   |
[01:32:38] 7 | let capacity = path.capacity();
[01:32:38]   |
[01:32:38]   |
[01:32:38]   = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:38] 
[01:32:38] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:38]   --> path.rs:1162:27
[01:32:38]    |
[01:32:38] 12 | assert_eq!(capacity, path.capacity());
[01:32:38]    |
[01:32:38]    |
[01:32:38]    = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:38] thread 'path.rs - path::PathBuf::with_capacity (line 1153)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:32:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:32:38] 
[01:32:38] 
---
[01:32:38] 
[01:32:38] error: test failed, to rerun pass '--doc'
[01:32:38] 
[01:32:38] 
[01:32:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:38] 
[01:32:38] 
[01:32:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:38] Build completed unsuccessfully in 0:32:00
[01:32:38] Build completed unsuccessfully in 0:32:00
[01:32:38] make: *** [check] Error 1
[01:32:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1757011e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 18 00:42:09 UTC 2019
---
travis_time:end:24430494:start=1550450530547432294,finish=1550450530552125071,duration=4692777
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30806070
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08a1101d
travis_time:start:08a1101d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:084c0776
$ dmesg | grep -i kill
