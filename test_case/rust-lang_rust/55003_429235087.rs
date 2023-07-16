plain
[00:54:59] .................................................................................................... 2200/4591
[00:55:03] .............i...................................................................................... 2300/4591
[00:55:07] .................................................................................................... 2400/4591
[00:55:10] .................................................................................................... 2500/4591
[00:55:14] ..........................iiiiiiiii................................................................. 2600/4591
[00:55:20] .................................................................................................... 2800/4591
[00:55:24] .................................................................................................... 2900/4591
[00:55:27] ..............................................i..................................................... 3000/4591
[00:55:30] .................................................................................................... 3100/4591
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:31] 
[01:08:31] running 111 tests
[01:08:34] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:08:34] ..iiii.....
[01:08:34] 
[01:08:34]  finished in 3.398
[01:08:34] travis_fold:end:test_codegen

---
[01:17:43] 
[01:17:43] 
[01:17:43] running 403 tests
[01:17:58] .................................................................................................... 100/403
[01:18:08] .....................................................................i........................F..... 200/403
[01:18:19] ...............F.................................................................................... 300/403
[01:18:28] ..................F...................F............................................................. 400/403
[01:18:28] failures:
[01:18:28] 
[01:18:28] ---- rc.rs - rc::Rc<T>::clone (line 865) stdout ----
[01:18:28] ---- rc.rs - rc::Rc<T>::clone (line 865) stdout ----
[01:18:28] error: unused return value of `std::clone::Clone::clone` which must be used
[01:18:28]  --> rc.rs:870:1
[01:18:28]   |
[01:18:28] 8 | Rc::clone(&five);
[01:18:28]   |
[01:18:28] note: lint level defined here
[01:18:28]  --> rc.rs:864:9
[01:18:28]   |
[01:18:28]   |
[01:18:28] 2 | #![deny(warnings)]
[01:18:28]   |         ^^^^^^^^
[01:18:28]   = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:18:28]   = note: cloning is often expensive and is not expected to have side effects
[01:18:28] thread 'rc.rs - rc::Rc<T>::clone (line 865)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:18:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:18:28] 
[01:18:28] ---- rc.rs - rc::Weak<T>::clone (line 1302) stdout ----
[01:18:28] ---- rc.rs - rc::Weak<T>::clone (line 1302) stdout ----
[01:18:28] error: unused return value of `std::clone::Clone::clone` which must be used
[01:18:28]  --> rc.rs:1307:1
[01:18:28]   |
[01:18:28] 8 | Weak::clone(&weak_five);
[01:18:28]   |
[01:18:28] note: lint level defined here
[01:18:28]  --> rc.rs:1301:9
[01:18:28]   |
[01:18:28]   |
[01:18:28] 2 | #![deny(warnings)]
[01:18:28]   |         ^^^^^^^^
[01:18:28]   = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:18:28]   = note: cloning is often expensive and is not expected to have side effects
[01:18:28] thread 'rc.rs - rc::Weak<T>::clone (line 1302)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:18:28] 
[01:18:28] ---- sync.rs - sync::Arc<T>::clone (line 711) stdout ----
[01:18:28] ---- sync.rs - sync::Arc<T>::clone (line 711) stdout ----
[01:18:28] error: unused return value of `std::clone::Clone::clone` which must be used
[01:18:28]  --> sync.rs:716:1
[01:18:28]   |
[01:18:28] 8 | Arc::clone(&five);
[01:18:28]   |
[01:18:28] note: lint level defined here
[01:18:28]  --> sync.rs:710:9
[01:18:28]   |
[01:18:28]   |
[01:18:28] 2 | #![deny(warnings)]
[01:18:28]   |         ^^^^^^^^
[01:18:28]   = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:18:28]   = note: cloning is often expensive and is not expected to have side effects
[01:18:28] thread 'sync.rs - sync::Arc<T>::clone (line 711)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:18:28] 
[01:18:28] ---- sync.rs - sync::Weak<T>::clone (line 1133) stdout ----
[01:18:28] ---- sync.rs - sync::Weak<T>::clone (line 1133) stdout ----
[01:18:28] error: unused return value of `std::clone::Clone::clone` which must be used
[01:18:28]  --> sync.rs:1138:1
[01:18:28]   |
[01:18:28] 8 | Weak::clone(&weak_five);
[01:18:28]   |
[01:18:28] note: lint level defined here
[01:18:28]  --> sync.rs:1132:9
[01:18:28]   |
[01:18:28]   |
[01:18:28] 2 | #![deny(warnings)]
[01:18:28]   |         ^^^^^^^^
[01:18:28]   = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:18:28]   = note: cloning is often expensive and is not expected to have side effects
[01:18:28] thread 'sync.rs - sync::Weak<T>::clone (line 1133)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:18:28] 
[01:18:28] 
[01:18:28] failures:
---
[01:18:28] 
[01:18:29] error: test failed, to rerun pass '--doc'
[01:18:29] 
[01:18:29] 
[01:18:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:18:29] 
[01:18:29] 
[01:18:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:29] Build completed unsuccessfully in 0:28:15
[01:18:29] Build completed unsuccessfully in 0:28:15
[01:18:29] make: *** [check] Error 1
[01:18:29] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0314f2be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
