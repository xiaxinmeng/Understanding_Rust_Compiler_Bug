plain
[00:45:56] .....i..............................................................................i...............
[00:46:01] ....................................................................................................
[00:46:07] ....................................................................................................
[00:46:13] ....................................................................................................
[00:46:18] .................i.................iiiiiiiii...................................................
[00:46:18] 
[00:46:18] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:47:08] .....i..............................................................................i...............
[00:47:13] ....................................................................................................
[00:47:18] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:28] .................i.................iiiiiiiii...................................................
[00:47:28] 
[00:47:28]  finished in 70.443
[00:47:28] travis_fold:end:test_ui_nll

---
[01:22:06] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:06]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:22:08] error[E0004]: non-exhaustive patterns: `ItemExistential(_)` not covered
[01:22:08]    --> librustc_driver/test.rs:250:26
[01:22:08] 250 |             return match it.node {
[01:22:08] 250 |             return match it.node {
[01:22:08]     |                          ^^^^^^^ pattern `ItemExistential(_)` not covered
[01:22:08] error: aborting due to previous error
[01:22:08] 
[01:22:08] For more information about this error, try `rustc --explain E0004`.
[01:22:08] error: Could not compile `rustc_driver`.
[01:22:08] error: Could not compile `rustc_driver`.
[01:22:08] 
[01:22:08] To learn more, run the command again with --verbose.
[01:22:08] 
[01:22:08] 
[01:22:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:22:08] 
[01:22:08] 
[01:22:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:08] Build completed unsuccessfully in 0:38:29
[01:22:08] Build completed unsuccessfully in 0:38:29
[01:22:08] make: *** [check] Error 1
[01:22:08] Makefile:58: recipe for target 'check' failed
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34872 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34372 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
