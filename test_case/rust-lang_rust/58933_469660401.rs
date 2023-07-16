plain
travis_time:end:0b3a6bf4:start=1551782993320238380,finish=1551782994396548827,duration=1076310447
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:46] 
[01:21:46] running 119 tests
[01:22:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:22:20] i......iii.i.....ii
[01:22:20] 
[01:22:20]  finished in 34.396
[01:22:20] travis_fold:end:test_debuginfo

---
[01:31:03] 
[01:31:04] 
[01:31:04] running 419 tests
[01:31:24] .................................................................................................... 100/419
[01:31:40] .............................................................................i..............F....... 200/419
[01:32:08] .................................................................................................... 400/419
[01:32:11] ...................
[01:32:11] failures:
[01:32:11] 
[01:32:11] 
[01:32:11] ---- prelude/mod.rs - prelude (line 6) stdout ----
[01:32:11] error[E0658]: use of unstable library feature 'alloc_prelude' (see issue #58935)
[01:32:11]  --> prelude/mod.rs:10:5
[01:32:11]   |
[01:32:11] 7 | use alloc::prelude::v1::*;
[01:32:11]   |
[01:32:11]   = help: add #![feature(alloc_prelude)] to the crate attributes to enable
[01:32:11] 
[01:32:11] thread 'prelude/mod.rs - prelude (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:352:13
---
[01:32:11] 
[01:32:11] error: test failed, to rerun pass '--doc'
[01:32:11] 
[01:32:11] 
[01:32:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:32:11] 
[01:32:11] 
[01:32:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:11] Build completed unsuccessfully in 0:23:11
[01:32:11] Build completed unsuccessfully in 0:23:11
[01:32:11] Makefile:48: recipe for target 'check' failed
[01:32:11] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0107cf04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar  5 12:22:16 UTC 2019
---
travis_time:end:0890a207:start=1551788538949106424,finish=1551788538955073105,duration=5966681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22c2b7ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0384bae4
travis_time:start:0384bae4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1fd297e4
$ dmesg | grep -i kill
