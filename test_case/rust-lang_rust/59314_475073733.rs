plain
travis_time:end:0ee2ca80:start=1553120295439445868,finish=1553120399184774265,duration=103745328397
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
[01:21:34] 
[01:21:34] running 9 tests
[01:21:34] iiiiiiiii
[01:21:34] 
[01:21:34]  finished in 0.157
[01:21:34] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:51] 
[01:21:51] running 120 tests
[01:22:18] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:23] .i......iii.i.....ii
[01:22:23] 
[01:22:23]  finished in 32.050
[01:22:23] travis_fold:end:test_debuginfo

---
[01:41:26] 
[01:41:26] running 997 tests
[01:41:50] i................................................................................................... 100/997
[01:42:04] .................................................................................................... 200/997
[01:42:14] ............iii......i......i...i.F.....i........................................................... 300/997
[01:42:28] ...........................i.i.....................................iiii........ii................... 500/997
[01:42:37] .................................................................................................... 600/997
[01:42:45] .................................................................................................... 700/997
[01:42:55] .................iiii............................................................................... 800/997
[01:42:55] .................iiii............................................................................... 800/997
[01:43:09] .................................................................................................... 900/997
[01:43:17] ...........................................iiii..................................................
[01:43:17] failures:
[01:43:17] 
[01:43:17] ---- ffi/c_str.rs - ffi::c_str::CString::from_reader (line 367) stdout ----
[01:43:17] error[E0658]: use of unstable library feature 'cstring_from_reader' (see issue #59229)
[01:43:17]  --> ffi/c_str.rs:371:14
[01:43:17]   |
[01:43:17] 7 | let string = CString::from_reader(test.as_bytes()).unwrap();
[01:43:17]   |
[01:43:17]   |
[01:43:17]   = help: add #![feature(cstring_from_reader)] to the crate attributes to enable
[01:43:17] error: aborting due to previous error
[01:43:17] 
[01:43:17] For more information about this error, try `rustc --explain E0658`.
[01:43:17] thread 'ffi/c_str.rs - ffi::c_str::CString::from_reader (line 367)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:43:17] 
[01:43:17] error: test failed, to rerun pass '--doc'
[01:43:17] 
[01:43:17] 
[01:43:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:43:17] 
[01:43:17] 
[01:43:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:17] Build completed unsuccessfully in 0:33:42
[01:43:17] Build completed unsuccessfully in 0:33:42
[01:43:17] make: *** [check] Error 1
[01:43:17] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a49840
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 21 00:03:26 UTC 2019
