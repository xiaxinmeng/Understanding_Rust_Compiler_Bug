plain
travis_time:end:0e0cc0ac:start=1544027612572084932,finish=1544027675108760406,duration=62536675474
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:56] 
[00:57:56] running 120 tests
[00:57:59] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:58:00] ..ii.i.....iiii.....
[00:58:00] 
[00:58:00]  finished in 3.525
[00:58:00] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:16] 
[00:58:16] running 118 tests
[00:58:40] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:58:44] ......iii.i.....ii
[00:58:44] 
[00:58:44]  finished in 28.661
[00:58:44] travis_fold:end:test_debuginfo

---
[01:08:16] error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[01:08:16] 
[01:08:16] 
[01:08:16] running 408 tests
[01:08:34] ...............................................F.................................................... 100/408
[01:09:02] .................................................................................................... 300/408
[01:09:14] .................................................................................................... 400/408
[01:09:15] ........
[01:09:15] failures:
[01:09:15] failures:
[01:09:15] 
[01:09:15] ---- collections/btree/map.rs - collections::btree::map::BTreeMap (line 69) stdout ----
[01:09:15] thread 'collections/btree/map.rs - collections::btree::map::BTreeMap (line 69)' panicked at 'test executable failed:
[01:09:15] We've got 4 reviews, but Les Misérables ain't one.
[01:09:15] Up! is unreviewed.
[01:09:15] Office Space: Deals with real issues in the workplace.
[01:09:15] 
[01:09:15] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[01:09:15] 
[01:09:15] ', src/librustdoc/test.rs:358:17
[01:09:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:09:15] 
---
[01:09:15] 
[01:09:15] error: test failed, to rerun pass '--doc'
[01:09:15] 
[01:09:15] 
[01:09:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:09:15] 
[01:09:15] 
[01:09:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:15] Build completed unsuccessfully in 0:22:26
[01:09:15] Build completed unsuccessfully in 0:22:26
[01:09:15] make: *** [check] Error 1
[01:09:15] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021cc3f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec  5 17:44:00 UTC 2018
---
travis_time:end:020925b6:start=1544031842064199093,finish=1544031842070011407,duration=5812314
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08b40f82
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/
