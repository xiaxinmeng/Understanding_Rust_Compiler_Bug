plain
travis_time:end:18499f1b:start=1542304911622934307,finish=1542304972295575162,duration=60672640855
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:55:21] .................................................................................................... 100/5021
[00:55:24] .................................................................................................... 200/5021
[00:55:27] .............................ii............................................ii...................ii.. 300/5021
[00:55:30] ..............................................................................................iii... 400/5021
[00:55:34] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:55:41] .................................................................................................... 700/5021
[00:55:48] .................................................................................i...........i...... 800/5021
[00:55:51] .................................................................................................... 900/5021
[00:55:55] iiiii..................ii.iiii...................................................................... 1000/5021
---
[00:56:35] .................................................................................................... 2200/5021
[00:56:40] .................................................................................................... 2300/5021
[00:56:44] .................................................................................................... 2400/5021
[00:56:48] .................................................................................................... 2500/5021
[00:56:51] .................................................................................iiiiiiiii.......... 2600/5021
[00:56:59] ...............................................ii................................................... 2800/5021
[00:57:03] .................................................................................................... 2900/5021
[00:57:07] .................................................................................................... 3000/5021
[00:57:10] ..........................................i......................................................... 3100/5021
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:03] 
[01:12:03] running 116 tests
[01:12:06] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:12:06] i.i....iiii.....
[01:12:06] 
[01:12:06]  finished in 3.808
[01:12:06] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:23] 
[01:12:23] running 118 tests
[01:12:50] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:12:54] ......iii.i.....ii
[01:12:54] 
[01:12:54]  finished in 31.617
[01:12:54] travis_fold:end:test_debuginfo

---
[01:24:58] 
[01:24:58] running 408 tests
[01:25:19] .................................................................................................... 100/408
[01:25:34] .....................................................................i.............................. 200/408
[01:25:49] ...................FF............................................................................... 300/408
[01:26:04] ........
[01:26:04] failures:
[01:26:04] 
[01:26:04] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1291) stdout ----
[01:26:04] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1291) stdout ----
[01:26:04] error[E0425]: cannot find value `third` in this scope
[01:26:04]   --> rc.rs:1300:13
[01:26:04]    |
[01:26:04] 12 | let third = third.downgrade();
[01:26:04] 
[01:26:04] 
[01:26:04] error[E0658]: use of unstable library feature 'weak_ptr_eq'
[01:26:04]  --> rc.rs:1297:10
[01:26:04]   |
[01:26:04] 9 | assert!(!Weak::ptr_eq(&first, &second));
[01:26:04]   |
[01:26:04]   |
[01:26:04]   = help: add #![feature(weak_ptr_eq)] to the crate attributes to enable
[01:26:04] 
[01:26:04] error[E0658]: use of unstable library feature 'weak_ptr_eq'
[01:26:04]   --> rc.rs:1302:10
[01:26:04]    |
[01:26:04] 14 | assert!(!Weak::ptr_eq(&first, &third));
[01:26:04]    |
[01:26:04]    |
[01:26:04]    = help: add #![feature(weak_ptr_eq)] to the crate attributes to enable
[01:26:04] thread 'rc.rs - rc::Weak<T>::ptr_eq (line 1291)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:26:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:04] 
[01:26:04] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1270) stdout ----
[01:26:04] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1270) stdout ----
[01:26:04] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:26:04]  --> rc.rs:1274:22
[01:26:04]   |
[01:26:04] 7 | let first = first_rc.downgrade();
[01:26:04]   |             |        |
[01:26:04]   |             |        this is an associated function, not a method
[01:26:04]   |             |        this is an associated function, not a method
[01:26:04]   |             help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:26:04]   |
[01:26:04]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:26:04]   = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:26:04] 
[01:26:04] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:26:04]  --> rc.rs:1275:23
[01:26:04]   |
[01:26:04] 8 | let second = first_rc.downgrade();
[01:26:04]   |              |        |
[01:26:04]   |              |        this is an associated function, not a method
[01:26:04]   |              |        this is an associated function, not a method
[01:26:04]   |              help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:26:04]   |
[01:26:04]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:26:04]   = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:26:04] 
[01:26:04] error[E0658]: use of unstable library feature 'weak_ptr_eq'
[01:26:04]   --> rc.rs:1277:9
[01:26:04]    |
[01:26:04] 10 | assert!(Weak::ptr_eq(&first, &second));
[01:26:04]    |
[01:26:04]    |
[01:26:04]    = help: add #![feature(weak_ptr_eq)] to the crate attributes to enable
[01:26:04] 
[01:26:04] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:26:04]   --> rc.rs:1280:22
[01:26:04]    |
[01:26:04] 13 | let third = third_rc.downgrade();
[01:26:04]    |             |        |
[01:26:04]    |             |        this is an associated function, not a method
[01:26:04]    |             |        this is an associated function, not a method
[01:26:04]    |             help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:26:04]    |
[01:26:04]    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:26:04]    = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:26:04] 
[01:26:04] error[E0658]: use of unstable library feature 'weak_ptr_eq'
[01:26:04]   --> rc.rs:1282:10
[01:26:04]    |
[01:26:04] 15 | assert!(!Weak::ptr_eq(&first, &third));
[01:26:04]    |
[01:26:04]    |
[01:26:04]    = help: add #![feature(weak_ptr_eq)] to the crate attributes to enable
[01:26:04] thread 'rc.rs - rc::Weak<T>::ptr_eq (line 1270)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:26:04] 
[01:26:04] 
[01:26:04] failures:
---
[01:26:04] 
[01:26:04] error: test failed, to rerun pass '--doc'
[01:26:04] 
[01:26:04] 
[01:26:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:26:04] 
[01:26:04] 
[01:26:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:04] Build completed unsuccessfully in 0:34:51
[01:26:04] Build completed unsuccessfully in 0:34:51
[01:26:04] Makefile:58: recipe for target 'check' failed
[01:26:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08f317a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 19:29:08 UTC 2018
