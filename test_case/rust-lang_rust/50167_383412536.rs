plain
[01:13:19] ....................................................................................................
[01:13:34] ....................................................................................................
[01:13:48] ....................................................................................................
[01:14:08] ....................................................................................................
[01:14:23] ....................i................................................F.................
[01:14:23] 
[01:14:23] 
[01:14:23] ---- time.rs - time::Duration::as_nanos (line 270) stdout ----
[01:14:23]  error[E0658]: use of unstable library feature 'duration_nanos' (see issue #50167)
[01:14:23]  --> time.rs:274:21
[01:14:23]   |
[01:14:23] 7 | assert_eq!(duration.as_nanos(), 5730023852);
[01:14:23]   |
[01:14:23]   |
[01:14:23]   = help: add #![feature(duration_nanos)] to the crate attributes to enable
[01:14:23] 
[01:14:23] thread 'time.rs - time::Duration::as_nanos (line 270)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:14:23] 
[01:14:23] 
[01:14:23] failures:
[01:14:23]     time.rs - time::Duration::as_nanos (line 270)
[01:14:23]     time.rs - time::Duration::as_nanos (line 270)
[01:14:23] 
[01:14:23] test result: FAILED. 1984 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:14:23] 
[01:14:23] error: test failed, to rerun pass '--doc'
[01:14:23] 
[01:14:23] 
[01:14:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:14:23] 
[01:14:23] 
[01:14:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:23] Build completed unsuccessfully in 0:34:30
[01:14:23] Build completed unsuccessfully in 0:34:30
[01:14:23] Makefile:58: recipe for target 'check' failed
[01:14:23] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b36015a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
