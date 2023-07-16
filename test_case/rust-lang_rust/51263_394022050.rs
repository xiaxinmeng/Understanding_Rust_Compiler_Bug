plain
[00:40:09]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:40:09]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:40:58] warning: [1] cannot be resolved, ignoring it...
[00:40:58] 
[00:40:59] warning: [::Poll] cannot be resolved, ignoring it...
[00:40:59] warning: [x] cannot be resolved, ignoring it...
[00:40:59] 
[00:40:59] warning: [] cannot be resolved, ignoring it...
[00:40:59] 
---
[00:41:09]     Checking unwind v0.0.0 (file:///checkout/src/libunwind)
[00:41:09]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:41:09]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:41:09]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:41:13] warning: [::Poll] cannot be resolved, ignoring it...
[00:41:13]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:41:13]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:41:13]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:41:13]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:41:13]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:41:13]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:41:13]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:41:24] warning: [::Poll] cannot be resolved, ignoring it...
[00:41:31]     Finished release [optimized] target(s) in 1m 23.26s
[00:41:32] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:41:32]     Checking getopts v0.2.17
[00:41:32]     Checking term v0.0.0 (file:///checkout/src/libterm)
---
[00:44:29] ..........................................................................i.........................
[00:44:34] ....................................................................................................
[00:44:40] ....................................................................................................
[00:44:46] ....................................................................................................
[00:44:50] ......i.................iiiiiiiii...................................................
[00:44:50] 
[00:44:50] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:45:41] ..........................................................................i.........................
[00:45:46] ....................................................................................................
[00:45:51] ....................................................................................................
[00:45:57] ....................................................................................................
[00:46:01] ......i.................iiiiiiiii...................................................
[00:46:01] 
[00:46:01]  finished in 71.302
[00:46:01] travis_fold:end:test_ui_nll

---
[01:08:00] error: no #[default_lib_allocator] found but one is required; is libstd not linked?
[01:08:00] 
[01:08:00] 
[01:08:00] running 398 tests
[01:08:19] .......................................................................FF...........................
[01:08:58] ....................................................................................................
[01:09:18] ..................................................................................................
[01:09:18] failures:
[01:09:18] 
[01:09:18] 
[01:09:18] ---- boxed.rs - boxed::PinBox<T>::from_raw (line 799) stdout ----
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:801:9
[01:09:18]   |
[01:09:18] 5 | let x = PinBox::new(5);
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:802:11
[01:09:18]   |
[01:09:18] 6 | let ptr = PinBox::into_raw(x);
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:803:18
[01:09:18]   |
[01:09:18] 7 | let x = unsafe { PinBox::from_raw(ptr) };
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:800:5
[01:09:18]   |
[01:09:18] 4 | use std::boxed::PinBox;
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] thread 'boxed.rs - boxed::PinBox<T>::from_raw (line 799)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:09:18] 
[01:09:18] 
[01:09:18] ---- boxed.rs - boxed::PinBox<T>::into_raw (line 826) stdout ----
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:828:9
[01:09:18]   |
[01:09:18] 5 | let x = PinBox::new(5);
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:829:11
[01:09:18]   |
[01:09:18] 6 | let ptr = PinBox::into_raw(x);
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] error[E0658]: use of unstable library feature 'pin' (see issue #49150)
[01:09:18]  --> boxed.rs:827:5
[01:09:18]   |
[01:09:18] 4 | use std::boxed::PinBox;
[01:09:18]   |
[01:09:18]   |
[01:09:18]   = help: add #![feature(pin)] to the crate attributes to enable
[01:09:18] 
[01:09:18] thread 'boxed.rs - boxed::PinBox<T>::into_raw (line 826)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:09:18] 
[01:09:18] failures:
[01:09:18] failures:
[01:09:18]     boxed.rs - boxed::PinBox<T>::from_raw (line 799)
[01:09:18]     boxed.rs - boxed::PinBox<T>::into_raw (line 826)
[01:09:18] test result: FAILED. 395 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
[01:09:18] 
[01:09:18] error: test failed, to rerun pass '--doc'
[01:09:19] 
[01:09:19] 
[01:09:19] 
[01:09:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:09:19] 
[01:09:19] 
[01:09:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:19] Build completed unsuccessfully in 0:27:05
[01:09:19] Build completed unsuccessfully in 0:27:05
[01:09:19] Makefile:58: recipe for target 'check' failed
[01:09:19] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12b827fe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
