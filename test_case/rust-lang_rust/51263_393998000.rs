plain
[00:43:20]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:43:20]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:44:08] warning: [1] cannot be resolved, ignoring it...
[00:44:08] 
[00:44:09] warning: [::Poll] cannot be resolved, ignoring it...
[00:44:09] warning: [x] cannot be resolved, ignoring it...
[00:44:09] 
[00:44:09] warning: [] cannot be resolved, ignoring it...
[00:44:09] 
---
[00:44:16]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:44:16]     Checking unwind v0.0.0 (file:///checkout/src/libunwind)
[00:44:16]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:44:16]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:44:20] warning: [::Poll] cannot be resolved, ignoring it...
[00:44:21]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:44:21]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:44:21]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:44:21]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:44:21]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:44:21]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:44:21]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:44:32] warning: [::Poll] cannot be resolved, ignoring it...
[00:44:39]     Finished release [optimized] target(s) in 1m 19.75s
[00:44:39] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:44:40]     Checking getopts v0.2.17
[00:44:40]     Checking term v0.0.0 (file:///checkout/src/libterm)
---
[00:47:39] ..........................................................................i.........................
[00:47:44] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:55] ....................................................................................................
[00:47:59] ......i.................iiiiiiiii...................................................
[00:47:59] 
[00:47:59] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:48] ..........................................................................i.........................
[00:48:53] ....................................................................................................
[00:48:57] ....................................................................................................
[00:49:03] ....................................................................................................
[00:49:07] ......i.................iiiiiii.ii..................................................
[00:49:07] 
[00:49:07]  finished in 67.783
[00:49:07] travis_fold:end:test_ui_nll

---
[01:10:32] error: no #[default_lib_allocator] found but one is required; is libstd not linked?
[01:10:32] 
[01:10:33] 
[01:10:33] running 398 tests
[01:10:52] .......................................................................FF...........................
[01:11:31] ....................................................................................................
[01:11:51] ..................................................................................................
[01:11:51] failures:
[01:11:51] 
[01:11:51] 
[01:11:51] ---- boxed.rs - boxed::PinBox<T>::from_raw (line 799) stdout ----
[01:11:51] error[E0433]: failed to resolve. Use of undeclared type or module `PinBox`
[01:11:51]  --> boxed.rs:800:9
[01:11:51]   |
[01:11:51] 4 | let x = PinBox::new(5);
[01:11:51]   |         ^^^^^^ Use of undeclared type or module `PinBox`
[01:11:51] error[E0433]: failed to resolve. Use of undeclared type or module `PinBox`
[01:11:51]  --> boxed.rs:801:11
[01:11:51]   |
[01:11:51]   |
[01:11:51] 5 | let ptr = PinBox::into_raw(x);
[01:11:51]   |           ^^^^^^ Use of undeclared type or module `PinBox`
[01:11:51] error[E0433]: failed to resolve. Use of undeclared type or module `PinBox`
[01:11:51]  --> boxed.rs:802:18
[01:11:51]   |
[01:11:51]   |
[01:11:51] 6 | let x = unsafe { PinBox::from_raw(ptr) };
[01:11:51]   |                  ^^^^^^ Use of undeclared type or module `PinBox`
[01:11:51] 
[01:11:51] thread 'boxed.rs - boxed::PinBox<T>::from_raw (line 799)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:11:51] 
[01:11:51] 
[01:11:51] ---- boxed.rs - boxed::PinBox<T>::into_raw (line 825) stdout ----
[01:11:51] error[E0433]: failed to resolve. Use of undeclared type or module `PinBox`
[01:11:51]  --> boxed.rs:826:9
[01:11:51]   |
[01:11:51] 4 | let x = PinBox::new(5);
[01:11:51]   |         ^^^^^^ Use of undeclared type or module `PinBox`
[01:11:51] error[E0433]: failed to resolve. Use of undeclared type or module `PinBox`
[01:11:51]  --> boxed.rs:827:11
[01:11:51]   |
[01:11:51]   |
[01:11:51] 5 | let ptr = PinBox::into_raw(x);
[01:11:51]   |           ^^^^^^ Use of undeclared type or module `PinBox`
[01:11:51] 
[01:11:51] thread 'boxed.rs - boxed::PinBox<T>::into_raw (line 825)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:11:51] 
[01:11:51] failures:
[01:11:51] failures:
[01:11:51]     boxed.rs - boxed::PinBox<T>::from_raw (line 799)
[01:11:51]     boxed.rs - boxed::PinBox<T>::into_raw (line 825)
[01:11:51] test result: FAILED. 395 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
[01:11:51] 
[01:11:51] error: test failed, to rerun pass '--doc'
[01:11:51] 
[01:11:51] 
[01:11:51] 
[01:11:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:11:51] 
[01:11:51] 
[01:11:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:51] Build completed unsuccessfully in 0:26:31
[01:11:51] Build completed unsuccessfully in 0:26:31
[01:11:51] make: *** [check] Error 1
[01:11:51] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2517b332
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
