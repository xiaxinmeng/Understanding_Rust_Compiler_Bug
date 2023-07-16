plain
travis_time:end:01705140:start=1549402094676692855,finish=1549402176981211283,duration=82304518428
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
[01:10:23] 
[01:10:23] running 119 tests
[01:10:49] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:54] i......iii.i.....ii
[01:10:54] 
[01:10:54]  finished in 31.129
[01:10:54] travis_fold:end:test_debuginfo

---
[01:22:01] ....i..iiiiii....................................................................................... 100/2240
[01:22:12] .................................................................................................... 200/2240
[01:22:25] ............................................................................i....................... 300/2240
[01:22:41] .................................................................................................... 400/2240
[01:22:53] .......................................FF........................................................... 500/2240
[01:23:18] .................................................................................................... 700/2240
[01:23:30] .................................................................................................... 800/2240
[01:23:43] .................................................................................................... 900/2240
[01:23:56] .................................................................................................... 1000/2240
---
[01:26:38] .......................................................i............................................ 2200/2240
[01:26:43] ...........i............................
[01:26:43] failures:
[01:26:43] 
[01:26:43] ---- mem.rs - mem::MaybeUninit (line 1045) stdout ----
[01:26:43] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[01:26:43]  --> mem.rs:1048:15
[01:26:43]   |
[01:26:43] 6 | let x: &i32 = mem::zeroed(); // undefined behavior!
[01:26:43]   |               ^^^^^^^^^^^^^ call to unsafe function
[01:26:43]   |
[01:26:43]   = note: consult the function's documentation for information on how to avoid undefined behavior
[01:26:43] 
[01:26:43] thread 'mem.rs - mem::MaybeUninit (line 1045)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:26:43] 
[01:26:43] 
[01:26:43] ---- mem.rs - mem::MaybeUninit (line 1060) stdout ----
[01:26:43] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[01:26:43]  --> mem.rs:1064:13
[01:26:43]   |
[01:26:43] 7 | let mut x = MaybeUninit::<&i32>::uninitialized();
[01:26:43]   |
[01:26:43]   = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[01:26:43] 
[01:26:43] 
[01:26:43] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[01:26:43]  --> mem.rs:1061:5
[01:26:43] 4 | use std::mem::MaybeUninit;
[01:26:43]   |     ^^^^^^^^^^^^^^^^^^^^^
[01:26:43]   |
[01:26:43]   = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[01:26:43]   = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[01:26:43] 
[01:26:43] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[01:26:43]  --> mem.rs:1066:3
[01:26:43]   |
[01:26:43] 9 | x.set(&0);
[01:26:43]   |
[01:26:43]   = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[01:26:43] 
[01:26:43] 
[01:26:43] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[01:26:43]   --> mem.rs:1069:20
[01:26:43]    |
[01:26:43] 12 | let x = unsafe { x.into_initialized() };
[01:26:43]    |
[01:26:43]    = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[01:26:43] 
[01:26:43] 
[01:26:43] thread 'mem.rs - mem::MaybeUninit (line 1060)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:26:43] 
[01:26:43] failures:
[01:26:43]     mem.rs - mem::MaybeUninit (line 1045)
[01:26:43]     mem.rs - mem::MaybeUninit (line 1060)
[01:26:43]     mem.rs - mem::MaybeUninit (line 1060)
[01:26:43] 
[01:26:43] test result: FAILED. 2228 passed; 2 failed; 10 ignored; 0 measured; 0 filtered out
[01:26:43] 
[01:26:43] error: test failed, to rerun pass '--doc'
[01:26:43] 
[01:26:43] 
[01:26:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:26:43] 
[01:26:43] 
[01:26:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:43] Build completed unsuccessfully in 0:28:01
[01:26:43] Build completed unsuccessfully in 0:28:01
[01:26:43] Makefile:48: recipe for target 'check' failed
[01:26:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:055300c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 22:56:29 UTC 2019
