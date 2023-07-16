plain
[01:08:24] 
[01:08:24]    Doc-tests core
[01:08:35] 
[01:08:35] running 1993 tests
[01:08:47] ..............................F.....................................................................
[01:09:17] ....................................................................................................
[01:09:32] .........................i..........................................................................
[01:09:43] ....................................................................................................
[01:09:55] ....................................................................................................
---
[01:13:12] ---- cell.rs - cell::Cell<T>::get_with (line 527) stdout ----
[01:13:12]  error[E0282]: type annotations needed
[01:13:12]  --> cell.rs:532:14
[01:13:12]   |
[01:13:12] 8 | assert_eq!(c.get_with(|v|v.iter().sum()), 6)
[01:13:12]   |              ^^^^^^^^ cannot infer type for `U`
[01:13:12] 
[01:13:12] thread 'cell.rs - cell::Cell<T>::get_with (line 527)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:13:12] 
[01:13:12] 
[01:13:12] failures:
[01:13:12]     cell.rs - cell::Cell<T>::get_with (line 527)
[01:13:12]     cell.rs - cell::Cell<T>::get_with (line 527)
[01:13:12] 
[01:13:12] test result: FAILED. 1990 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:13:12] 
[01:13:12] error: test failed, to rerun pass '--doc'
[01:13:12] 
[01:13:12] 
[01:13:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:13:12] 
[01:13:12] 
[01:13:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:12] Build completed unsuccessfully in 0:32:18
[01:13:12] Build completed unsuccessfully in 0:32:18
[01:13:12] make: *** [check] Error 1
[01:13:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17e61ae2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
