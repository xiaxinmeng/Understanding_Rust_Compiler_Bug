plain
[00:47:11] ................................................................i...................................
[00:47:15] ....................................................................................................
[00:47:20] ....................................................................................................
[00:47:27] .............................................................................................i......
[00:47:29] ...........iiiiiiiii...................................................
[00:47:29] 
[00:47:29] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:16] ................................................................i...................................
[00:48:20] ....................................................................................................
[00:48:25] ....................................................................................................
[00:48:31] .............................................................................................i......
[00:48:34] ...........iiiiiiiii...................................................
[00:48:34] 
[00:48:34]  finished in 64.264
[00:48:34] travis_fold:end:test_ui_nll

---
[01:10:23] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:23]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:10:32] error[E0599]: no method named `div_euc` found for type `{float}` in the current scope
[01:10:32]    --> libcore/../libcore/tests/num/mod.rs:569:26
[01:10:32]     |
[01:10:32] 569 |             assert_eq!(a.div_euc($inf), 0.0);
[01:10:32] ...
[01:10:32] ...
[01:10:32] 578 | test_float!(f32, f32, ::core::f32::INFINITY, ::core::f32::NEG_INFINITY, ::core::f32::NAN);
[01:10:32]     | ------------------------------------------------------------------------------------------ in this macro invocation
[01:10:32] 
[01:10:32] error[E0599]: no method named `div_euc` found for type `{float}` in the current scope
[01:10:32]    --> libcore/../libcore/tests/num/mod.rs:570:23
[01:10:32]     |
[01:10:32] 570 |             assert!(a.div_euc($nan).is_nan());
[01:10:32] ...
[01:10:32] ...
[01:10:32] 578 | test_float!(f32, f32, ::core::f32::INFINITY, ::core::f32::NEG_INFINITY, ::core::f32::NAN);
[01:10:32]     | ------------------------------------------------------------------------------------------ in this macro invocation
[01:10:32] 
[01:10:32] error[E0599]: no method named `div_euc` found for type `{float}` in the current scope
[01:10:32]    --> libcore/../libcore/tests/num/mod.rs:569:26
[01:10:32]     |
[01:10:32] 569 |             assert_eq!(a.div_euc($inf), 0.0);
[01:10:32] ...
[01:10:32] ...
[01:10:32] 579 | test_float!(f64, f64, ::core::f64::INFINITY, ::core::f64::NEG_INFINITY, ::core::f64::NAN);
[01:10:32]     | ------------------------------------------------------------------------------------------ in this macro invocation
[01:10:32] 
[01:10:32] error[E0599]: no method named `div_euc` found for type `{float}` in the current scope
[01:10:32]    --> libcore/../libcore/tests/num/mod.rs:570:23
[01:10:32]     |
[01:10:32] 570 |             assert!(a.div_euc($nan).is_nan());
[01:10:32] ...
[01:10:32] ...
[01:10:32] 579 | test_float!(f64, f64, ::core::f64::INFINITY, ::core::f64::NEG_INFINITY, ::core::f64::NAN);
[01:10:32]     | ------------------------------------------------------------------------------------------ in this macro invocation
[01:10:37] error: aborting due to 4 previous errors
[01:10:37] 
[01:10:37] For more information about this error, try `rustc --explain E0599`.
[01:10:37] error: Could not compile `core`.
[01:10:37] error: Could not compile `core`.
[01:10:37] 
[01:10:37] To learn more, run the command again with --verbose.
[01:10:37] 
[01:10:37] 
[01:10:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:10:37] 
[01:10:37] 
[01:10:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:37] Build completed unsuccessfully in 0:25:41
[01:10:37] Build completed unsuccessfully in 0:25:41
[01:10:37] make: *** [check] Error 1
[01:10:37] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0648d40a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
