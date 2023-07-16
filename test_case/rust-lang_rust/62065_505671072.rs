plain
travis_time:end:0a71704a:start=1561502802656066164,finish=1561502892898002966,duration=90241936802
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:24] 
[01:24:24] running 143 tests
[01:24:26] i..iii.....iii..iiii.....i......................i..i.................i......i.........ii.i..i..i.ii. 100/143
[01:24:28] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:24:28] 
[01:24:28]  finished in 4.708
[01:24:28] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:30] 
[01:24:30] running 9 tests
[01:24:30] iiiiiiiii
[01:24:30] 
[01:24:30]  finished in 0.155
[01:24:30] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:47] 
[01:24:47] running 122 tests
[01:25:13] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:25:18] .i.i......iii.i.....ii
[01:25:18] 
[01:25:18]  finished in 31.713
[01:25:18] travis_fold:end:test_debuginfo

---
[01:37:48] ....iiiii........................................................................................... 100/2323
[01:38:00] .......................................................................ii........................... 200/2323
[01:38:14] ...........................................................................................i........ 300/2323
[01:38:31] .................................................................................................... 400/2323
[01:38:43] .......................i..i....................................F.................................... 500/2323
[01:39:09] .................................................................................................... 700/2323
[01:39:22] .................................................................................................... 800/2323
[01:39:35] .................................................................................................... 900/2323
[01:39:48] .................................................................................................... 1000/2323
---
[01:42:56] ---- mem.rs - mem::MaybeUninit (line 1073) stdout ----
[01:42:56] error: unused import: `align_of`
[01:42:56]  --> mem.rs:1074:38
[01:42:56]   |
[01:42:56] 4 | use std::mem::{MaybeUninit, size_of, align_of};
[01:42:56]   |
[01:42:56] note: lint level defined here
[01:42:56]  --> mem.rs:1071:9
[01:42:56]   |
---
[01:42:56] 
[01:42:56] error: test failed, to rerun pass '--doc'
[01:42:56] 
[01:42:56] 
[01:42:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:42:56] 
[01:42:56] 
[01:42:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:56] Build completed unsuccessfully in 0:30:39
[01:42:56] Build completed unsuccessfully in 0:30:39
[01:42:56] Makefile:48: recipe for target 'check' failed
[01:42:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:127aa424
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jun 26 00:31:19 UTC 2019
---
travis_time:end:2b68f1f5:start=1561509081498620587,finish=1561509081565649293,duration=67028706
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d1602c2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:173484f0
$ dmesg | grep -i kill
