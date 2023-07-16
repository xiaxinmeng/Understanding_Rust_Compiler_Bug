plain
travis_time:end:0c9b8eaf:start=1551715561873205141,finish=1551715563775151375,duration=1901946234
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
[01:20:17] 
[01:20:17] running 119 tests
[01:20:46] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:20:51] i......iii.i.....ii
[01:20:51] 
[01:20:51]  finished in 33.384
[01:20:51] travis_fold:end:test_debuginfo

---
[01:39:01] .................................................................................................... 100/774
[01:39:01] .................................................................................................... 200/774
[01:39:01] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1339:17
[01:39:01] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:805:13
[01:39:01] ......................................................F............................................. 300/774
[01:39:02] .................................................................................................... 500/774
[01:39:02] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:997:5
[01:39:02] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:997:5
[01:39:02] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:997:5
---
[01:39:13] failures:
[01:39:13] 
[01:39:13] ---- io::buffered::tests::test_buffered_reader_seek_relative_read stdout ----
[01:39:13] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:39:13]   left: `[6, 7]`,
[01:39:13]  right: `[3, 4]`', src/libstd/io/buffered.rs:1087:9
[01:39:13] 
[01:39:13] failures:
[01:39:13]     io::buffered::tests::test_buffered_reader_seek_relative_read
[01:39:13] 
[01:39:13] 
[01:39:13] test result: FAILED. 773 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:39:13] 
[01:39:13] error: test failed, to rerun pass '--lib'
[01:39:13] 
[01:39:13] 
[01:39:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:39:13] 
[01:39:13] 
[01:39:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:13] Build completed unsuccessfully in 0:31:25
[01:39:13] Build completed unsuccessfully in 0:31:25
[01:39:13] Makefile:48: recipe for target 'check' failed
[01:39:13] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0019d106
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 17:45:28 UTC 2019
---
travis_time:end:0011346a:start=1551721530381457301,finish=1551721530386629441,duration=5172140
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036a4ca4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(e
