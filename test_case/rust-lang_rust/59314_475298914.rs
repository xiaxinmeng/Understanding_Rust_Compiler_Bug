plain
travis_time:end:1e8cd399:start=1553178190718059071,finish=1553178268014020174,duration=77295961103
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
[01:30:05] 
[01:30:05] running 9 tests
[01:30:05] iiiiiiiii
[01:30:05] 
[01:30:05]  finished in 0.163
[01:30:05] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:30:23] 
[01:30:23] running 120 tests
[01:30:52] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:30:57] .i......iii.i.....ii
[01:30:57] 
[01:30:57]  finished in 34.059
[01:30:57] travis_fold:end:test_debuginfo

---
[01:51:07] 
[01:51:07] running 997 tests
[01:51:33] i................................................................................................... 100/997
[01:51:48] .................................................................................................... 200/997
[01:51:58] ............iii......i......i...i.F.....i........................................................... 300/997
[01:52:13] ...........................i.i.....................................iiii........ii................... 500/997
[01:52:22] .................................................................................................... 600/997
[01:52:30] .................................................................................................... 700/997
[01:52:39] .................iiii............................................................................... 800/997
[01:52:39] .................iiii............................................................................... 800/997
[01:52:55] .................................................................................................... 900/997
[01:53:04] ...........................................iiii..................................................
[01:53:04] failures:
[01:53:04] 
[01:53:04] ---- ffi/c_str.rs - ffi::c_str::CString::from_reader (line 367) stdout ----
[01:53:04] error[E0658]: use of unstable library feature 'cstring_from_reader' (see issue #59229)
[01:53:04]  --> ffi/c_str.rs:371:14
[01:53:04]   |
[01:53:04] 7 | let string = CString::from_reader(test.as_bytes()).unwrap();
[01:53:04]   |
[01:53:04]   |
[01:53:04]   = help: add #![feature(cstring_from_reader)] to the crate attributes to enable
[01:53:04] error: aborting due to previous error
[01:53:04] 
[01:53:04] For more information about this error, try `rustc --explain E0658`.
[01:53:04] thread 'ffi/c_str.rs - ffi::c_str::CString::from_reader (line 367)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:53:04] 
[01:53:04] error: test failed, to rerun pass '--doc'
[01:53:04] 
[01:53:04] 
[01:53:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:53:04] 
[01:53:04] 
[01:53:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:04] Build completed unsuccessfully in 0:36:28
[01:53:04] Build completed unsuccessfully in 0:36:28
[01:53:04] make: *** [check] Error 1
[01:53:04] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0491a5d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 21 16:17:42 UTC 2019
