plain
[00:45:56] .................................................................................................i.. 2200/4552
[00:46:01] .................................................................................................... 2300/4552
[00:46:04] .................................................................................................... 2400/4552
[00:46:07] .................................................................................................... 2500/4552
[00:46:11] .........iiiiiiiii.................................................................................. 2600/4552
[00:46:16] .................................................................................................... 2800/4552
[00:46:20] .................................................................................................... 2900/4552
[00:46:23] ............................i....................................................................... 3000/4552
[00:46:25] ........................................................................................i.i..ii..... 3100/4552
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:09] 
[00:58:09] running 107 tests
[00:58:12] i..ii...iii....i...i.........i..iii...........i.....i....ii...i.i.ii..............i...ii..ii.i....ii 100/107
[00:58:12] test result: ok. 77 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[00:58:12] 
[00:58:12]  finished in 3.193
[00:58:12] travis_fold:end:test_codegen
---
[01:10:39] .................................................................................................... 1500/2166
[01:10:45] .................................................................................................... 1600/2166
[01:10:52] .................................................................................................... 1700/2166
[01:11:00] .................................................................................................... 1800/2166
[01:11:07] ..................F................................................................................. 1900/2166
[01:11:25] ..................................................................................i................. 2100/2166
[01:11:30] .....................................i............................
[01:11:30] failures:
[01:11:30] 
[01:11:30] 
[01:11:30] ---- ops/unsize.rs - ops::unsize::DispatchFromDyn (line 86) stdout ----
[01:11:30] error[E0405]: cannot find trait `DispatchFromDyn` in this scope
[01:11:30]  --> ops/unsize.rs:87:28
[01:11:30]   |
[01:11:30] 4 | impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Rc<U>> for Rc<T>
[01:11:30] help: possible candidate is found in another module, you can import it into scope
[01:11:30]   |
[01:11:30]   |
[01:11:30] 3 | use std::ops::DispatchFromDyn;
[01:11:30] 
[01:11:30] error[E0412]: cannot find type `Rc` in this scope
[01:11:30]  --> ops/unsize.rs:87:44
[01:11:30]   |
[01:11:30]   |
[01:11:30] 4 | impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Rc<U>> for Rc<T>
[01:11:30]   |                                            ^^ not found in this scope
[01:11:30]   |
[01:11:30] 3 | use std::rc::Rc;
[01:11:30]   |
[01:11:30] 
[01:11:30] 
[01:11:30] error[E0412]: cannot find type `Rc` in this scope
[01:11:30]  --> ops/unsize.rs:87:55
[01:11:30]   |
[01:11:30] 4 | impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Rc<U>> for Rc<T>
[01:11:30]   |                                                       ^^ not found in this scope
[01:11:30]   |
[01:11:30] 3 | use std::rc::Rc;
[01:11:30]   |
[01:11:30] 
[01:11:30] 
[01:11:30] error[E0405]: cannot find trait `Unsize` in this scope
[01:11:30]  --> ops/unsize.rs:89:8
[01:11:30]   |
[01:11:30] 6 |     T: Unsize<U>,
[01:11:30] help: possible candidate is found in another module, you can import it into scope
[01:11:30]   |
[01:11:30] 3 | use std::marker::Unsize;
[01:11:30]   |
[01:11:30]   |
[01:11:30] 
[01:11:30] thread 'ops/unsize.rs - ops::unsize::DispatchFromDyn (line 86)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:11:30] 
[01:11:30] 
[01:11:30] failures:
[01:11:30] failures:
[01:11:30]     ops/unsize.rs - ops::unsize::DispatchFromDyn (line 86)
[01:11:30] test result: FAILED. 2162 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:11:30] 
[01:11:30] error: test failed, to rerun pass '--doc'
[01:11:30] 
[01:11:30] 
[01:11:30] 
[01:11:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:11:30] 
[01:11:30] 
[01:11:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:30] Build completed unsuccessfully in 0:29:55
[01:11:30] Build completed unsuccessfully in 0:29:55
[01:11:30] Makefile:58: recipe for target 'check' failed
[01:11:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05be68ce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05816295:start=1538643468589890657,finish=1538643468594941308,duration=5050651
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:080e3e78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\
