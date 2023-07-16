plain
[00:53:19] ....i...................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:53:25] ............
[00:53:58] ....................................................................................................
[00:54:28] ......................................................................ii............................
[00:55:20] .................................i....................................................i.ii......test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:05] ..............................................................................................iiiiii
[00:56:33] i...................................................................................................
[00:57:04] ....................................................................................................
[00:57:33] ....................................................................................................
---
[01:28:12] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:13]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:28:14] error[E0423]: expected function, found struct `ty::Binder`
[01:28:14]    --> librustc_driver/test.rs:287:34
[01:28:14]     |
[01:28:14] 287 |         self.infcx.tcx.mk_fn_ptr(ty::Binder(self.infcx.tcx.mk_fn_sig(
[01:28:14]     |                                  ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[01:28:15] error: aborting due to previous error
[01:28:15] 
[01:28:15] For more information about this error, try `rustc --explain E0423`.
[01:28:15] error: Could not compile `rustc_driver`.
[01:28:15] error: Could not compile `rustc_driver`.
[01:28:15] 
[01:28:15] To learn more, run the command again with --verbose.
[01:28:15] 
[01:28:15] 
[01:28:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:28:15] 
[01:28:15] 
[01:28:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:15] Build completed unsuccessfully in 0:44:18
[01:28:15] Build completed unsuccessfully in 0:44:18
[01:28:15] Makefile:58: recipe for target 'check' failed
[01:28:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002718f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
