plain
[00:54:13] ........................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:33] ............................................................................
[00:54:53] ......................................................................................ii............
[00:55:44] ..................................................i.................................................
[00:55:55] ...i.ii..........................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:39] ...........iiiiiii..................................................................................
[00:56:59] ....................................................................................................
[00:57:16] ....................................................................................................
[00:57:35] ....................................................................................
---
[01:12:26] ....................................................................................................
[01:12:39] ....................................................................................................
[01:12:56] ....................................................................................................
[01:13:11] ........................i...........................................................................
[01:13:22] ............................F.F.............F..F....................................................
[01:13:46] ....................................................................................................
[01:13:59] ....................................................................................................
[01:14:11] ....................................................................................................
[01:14:23] ....................................................................................................
---
[01:16:43] ...................................i................................................................
[01:16:44] .
[01:16:44] failures:
[01:16:44] 
[01:16:44] ---- num/f32.rs - f32::f32::compensated_add (line 432) stdout ----
[01:16:44]  error[E0599]: no method named `compensated_add` found for type `{float}` in the current scope
[01:16:44]  --> num/f32.rs:438:27
[01:16:44]   |
[01:16:44] 9 |     let (s, c) = comp_sum.compensated_add(x, comp);
[01:16:44] 
[01:16:44] 
[01:16:44] thread 'num/f32.rs - f32::f32::compensated_add (line 432)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:16:44] 
[01:16:44] 
[01:16:44] ---- num/f32.rs - f32::f32::compensated_add (line 449) stdout ----
[01:16:44]  error[E0599]: no method named `compensated_add` found for type `{float}` in the current scope
[01:16:44]  --> num/f32.rs:455:23
[01:16:44]   |
[01:16:44] 9 |         |(s, c), x| s.compensated_add(x, c)
[01:16:44] 
[01:16:44] 
[01:16:44] thread 'num/f32.rs - f32::f32::compensated_add (line 449)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:16:44] 
[01:16:44] ---- num/f64.rs - f64::f64::compensated_add (line 443) stdout ----
[01:16:44]  error[E0599]: no method named `compensated_add` found for type `{float}` in the current scope
[01:16:44]  --> num/f64.rs:449:27
[01:16:44]   |
[01:16:44] 9 |     let (s, c) = comp_sum.compensated_add(x, comp);
[01:16:44] 
[01:16:44] 
[01:16:44] thread 'num/f64.rs - f64::f64::compensated_add (line 443)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:16:44] 
[01:16:44] ---- num/f64.rs - f64::f64::compensated_add (line 460) stdout ----
[01:16:44]  error[E0599]: no method named `compensated_add` found for type `{float}` in the current scope
[01:16:44]  --> num/f64.rs:466:23
[01:16:44]   |
[01:16:44] 9 |         |(s, c), x| s.compensated_add(x, c)
[01:16:44] 
[01:16:44] 
[01:16:44] thread 'num/f64.rs - f64::f64::compensated_add (line 460)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:16:44] 
[01:16:44] failures:
[01:16:44] failures:
[01:16:44]     num/f32.rs - f32::f32::compensated_add (line 432)
[01:16:44]     num/f32.rs - f32::f32::compensated_add (line 449)
[01:16:44]     num/f64.rs - f64::f64::compensated_add (line 443)
[01:16:44]     num/f64.rs - f64::f64::compensated_add (line 460)
[01:16:44] test result: FAILED. 1995 passed; 4 failed; 2 ignored; 0 measured; 0 filtered out
[01:16:44] 
[01:16:44] error: test failed, to rerun pass '--doc'
[01:16:44] 
[01:16:44] 
[01:16:44] 
[01:16:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:16:44] 
[01:16:44] 
[01:16:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:44] Build completed unsuccessfully in 0:32:25
[01:16:44] Build completed unsuccessfully in 0:32:25
[01:16:44] Makefile:58: recipe for target 'check' failed
[01:16:44] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b33c6ff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
