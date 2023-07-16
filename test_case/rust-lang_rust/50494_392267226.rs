plain
[00:51:03] ..........................................................i.........................................
[00:51:07] ....................................................................................................
[00:51:13] ....................................................................................................
[00:51:20] .....................................................................................i..............
[00:51:22] ...iiiiiiiii...................................................
[00:51:22] 
[00:51:22] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:13] ..........................................................i.........................................
[00:52:18] ....................................................................................................
[00:52:23] ....................................................................................................
[00:52:29] .....................................................................................i..............
[00:52:32] ...iiiiiiiii...................................................
[00:52:32] 
[00:52:32]  finished in 69.451
[00:52:32] travis_fold:end:test_ui_nll

---
[01:17:52] 
[01:17:52]    Doc-tests core
[01:18:04] 
[01:18:04] running 1989 tests
[01:18:17] ..........................F.........................................................................
[01:18:49] ....................................................................................................
[01:19:06] .......................i............................................................................
[01:19:18] ....................................................................................................
[01:19:31] ....................................................................................................
---
[01:22:38] ....................................................................................................
[01:22:55] .......................i.................................................................
[01:22:55] failures:
[01:22:55] 
[01:22:55] ---- cell.rs - cell::Cell<T>::from_mut (line 508) stdout ----
[01:22:55] error: expected one of `.`, `;`, `?`, or an operator, found `assert_eq`
[01:22:55]  --> cell.rs:514:1
[01:22:55]   |
[01:22:55] 8 | let slice_cell : &[Cell<i32>] = &cell_slice[..]
[01:22:55]   |                                                - expected one of `.`, `;`, `?`, or an operator here
[01:22:55] 9 | assert_eq!(slice_cell.len(), 3)
[01:22:55]   | ^^^^^^^^^ unexpected token
[01:22:55] thread 'cell.rs - cell::Cell<T>::from_mut (line 508)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:22:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:55] 
[01:22:55] 
---
[01:22:55] 
[01:22:55] error: test failed, to rerun pass '--doc'
[01:22:55] 
[01:22:55] 
[01:22:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:22:55] 
[01:22:55] 
[01:22:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:55] Build completed unsuccessfully in 0:34:23
[01:22:55] Build completed unsuccessfully in 0:34:23
[01:22:55] Makefile:58: recipe for target 'check' failed
[01:22:55] make: *** [check] Error 1
68788 ./src/llvm/lib
65416 ./src/llvm-emscripten/test/CodeGen
62732 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
61608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
---
travis_fold:start:after_failure.4
travis_time:start:058c0a94
$ dmesg | grep -i kill
[   11.432730] init: failsafe main process (1092) killed by TERM signal
[   42.898417] init: plymouth-upstart-bridge main process (509) killed by TERM signal
travis_fold:end:after_failure.4

Done. Your build exited with 1.
