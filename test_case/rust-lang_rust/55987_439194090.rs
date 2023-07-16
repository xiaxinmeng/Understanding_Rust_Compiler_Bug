plain
travis_time:end:0e54aaee:start=1542311928272529443,finish=1542311981984741777,duration=53712212334
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:26] .................................................................................................... 100/5021
[00:51:29] .................................................................................................... 200/5021
[00:51:32] .............................ii............................................ii...................ii.. 300/5021
[00:51:35] ..............................................................................................iii... 400/5021
[00:51:38] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:51:45] .................................................................................................... 700/5021
[00:51:51] .................................................................................i...........i...... 800/5021
[00:51:55] .................................................................................................... 900/5021
[00:51:58] iiiii..................ii.iiii...................................................................... 1000/5021
---
[00:52:34] .................................................................................................... 2200/5021
[00:52:38] .................................................................................................... 2300/5021
[00:52:42] .................................................................................................... 2400/5021
[00:52:46] .................................................................................................... 2500/5021
[00:52:49] .................................................................................iiiiiiiii.......... 2600/5021
[00:52:56] ................................................ii.................................................. 2800/5021
[00:52:59] .................................................................................................... 2900/5021
[00:53:03] .................................................................................................... 3000/5021
[00:53:06] ..........................................i......................................................... 3100/5021
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:56] 
[01:06:56] running 116 tests
[01:06:59] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:07:00] i.i....iiii.....
[01:07:00] 
[01:07:00]  finished in 3.545
[01:07:00] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:14] 
[01:07:14] running 118 tests
[01:07:39] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:07:43] ......iii.i.....ii
[01:07:43] 
[01:07:43]  finished in 29.172
[01:07:43] travis_fold:end:test_debuginfo

---
[01:19:02] 
[01:19:02] running 410 tests
[01:19:20] .................................................................................................... 100/410
[01:19:34] .....................................................................i.............................. 200/410
[01:19:46] ...................F.F.............................................................................. 300/410
[01:19:58] ...............................................FF................................................... 400/410
[01:19:59] failures:
[01:19:59] 
[01:19:59] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1270) stdout ----
[01:19:59] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1270) stdout ----
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:19:59]  --> rc.rs:1275:22
[01:19:59]   |
[01:19:59] 8 | let first = first_rc.downgrade();
[01:19:59]   |             |        |
[01:19:59]   |             |        this is an associated function, not a method
[01:19:59]   |             |        this is an associated function, not a method
[01:19:59]   |             help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:19:59]   |
[01:19:59]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]   = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:19:59] 
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:19:59]  --> rc.rs:1276:23
[01:19:59]   |
[01:19:59] 9 | let second = first_rc.downgrade();
[01:19:59]   |              |        |
[01:19:59]   |              |        this is an associated function, not a method
[01:19:59]   |              |        this is an associated function, not a method
[01:19:59]   |              help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:19:59]   |
[01:19:59]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]   = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:19:59] 
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<{integer}>` in the current scope
[01:19:59]   --> rc.rs:1281:22
[01:19:59]    |
[01:19:59] 14 | let third = third_rc.downgrade();
[01:19:59]    |             |        |
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             help: use associated function syntax instead: `std::rc::Rc<{integer}>::downgrade`
[01:19:59]    |
[01:19:59]    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]    = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:19:59] thread 'rc.rs - rc::Weak<T>::ptr_eq (line 1270)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:19:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:59] 
[01:19:59] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1292) stdout ----
[01:19:59] ---- rc.rs - rc::Weak<T>::ptr_eq (line 1292) stdout ----
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::rc::Rc<()>` in the current scope
[01:19:59]   --> rc.rs:1302:22
[01:19:59]    |
[01:19:59] 13 | let third = third_rc.downgrade();
[01:19:59]    |             |        |
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             help: use associated function syntax instead: `std::rc::Rc<()>::downgrade`
[01:19:59]    |
[01:19:59]    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]    = note: the candidate is defined in an impl for the type `std::rc::Rc<_>`
[01:19:59] thread 'rc.rs - rc::Weak<T>::ptr_eq (line 1292)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:19:59] 
[01:19:59] ---- sync.rs - sync::Weak<T>::ptr_eq (line 1140) stdout ----
[01:19:59] ---- sync.rs - sync::Weak<T>::ptr_eq (line 1140) stdout ----
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::sync::Arc<{integer}>` in the current scope
[01:19:59]  --> sync.rs:1145:22
[01:19:59]   |
[01:19:59] 8 | let first = first_rc.downgrade();
[01:19:59]   |             |        |
[01:19:59]   |             |        this is an associated function, not a method
[01:19:59]   |             |        this is an associated function, not a method
[01:19:59]   |             help: use associated function syntax instead: `std::sync::Arc<{integer}>::downgrade`
[01:19:59]   |
[01:19:59]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]   = note: the candidate is defined in an impl for the type `std::sync::Arc<_>`
[01:19:59] 
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::sync::Arc<{integer}>` in the current scope
[01:19:59]  --> sync.rs:1146:23
[01:19:59]   |
[01:19:59] 9 | let second = first_rc.downgrade();
[01:19:59]   |              |        |
[01:19:59]   |              |        this is an associated function, not a method
[01:19:59]   |              |        this is an associated function, not a method
[01:19:59]   |              help: use associated function syntax instead: `std::sync::Arc<{integer}>::downgrade`
[01:19:59]   |
[01:19:59]   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]   = note: the candidate is defined in an impl for the type `std::sync::Arc<_>`
[01:19:59] 
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::sync::Arc<{integer}>` in the current scope
[01:19:59]   --> sync.rs:1151:22
[01:19:59]    |
[01:19:59] 14 | let third = third_rc.downgrade();
[01:19:59]    |             |        |
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             help: use associated function syntax instead: `std::sync::Arc<{integer}>::downgrade`
[01:19:59]    |
[01:19:59]    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]    = note: the candidate is defined in an impl for the type `std::sync::Arc<_>`
[01:19:59] thread 'sync.rs - sync::Weak<T>::ptr_eq (line 1140)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:19:59] 
[01:19:59] ---- sync.rs - sync::Weak<T>::ptr_eq (line 1162) stdout ----
[01:19:59] ---- sync.rs - sync::Weak<T>::ptr_eq (line 1162) stdout ----
[01:19:59] error[E0599]: no method named `downgrade` found for type `std::sync::Arc<()>` in the current scope
[01:19:59]   --> sync.rs:1172:22
[01:19:59]    |
[01:19:59] 13 | let third = third_rc.downgrade();
[01:19:59]    |             |        |
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             |        this is an associated function, not a method
[01:19:59]    |             help: use associated function syntax instead: `std::sync::Arc<()>::downgrade`
[01:19:59]    |
[01:19:59]    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[01:19:59]    = note: the candidate is defined in an impl for the type `std::sync::Arc<_>`
[01:19:59] thread 'sync.rs - sync::Weak<T>::ptr_eq (line 1162)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:19:59] 
[01:19:59] 
[01:19:59] failures:
---
[01:19:59] 
[01:19:59] error: test failed, to rerun pass '--doc'
[01:19:59] 
[01:19:59] 
[01:19:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:19:59] 
[01:19:59] 
[01:19:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:59] Build completed unsuccessfully in 0:32:21
[01:19:59] Build completed unsuccessfully in 0:32:21
[01:19:59] Makefile:58: recipe for target 'check' failed
[01:19:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2350481c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 21:19:52 UTC 2018
