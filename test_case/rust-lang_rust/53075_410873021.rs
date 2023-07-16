plain
[00:46:06] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:17] ......iiiiiiiii.....................................................................................
[00:46:23] ....................................................................................................
[00:46:27] ...........i........................................................................................
[00:46:30] ....................i...............................................................................
[00:46:33] ....................................................................................................
---
[01:00:19]    Compiling rand_core v0.2.1
[01:00:19]    Compiling libc v0.2.42
[01:00:21]    Compiling rand v0.5.4
[01:00:24]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:00:27] error[E0089]: too few type parameters provided: expected 2 type parameters, found 1 type parameter
[01:00:27]     |
[01:00:27]     |
[01:00:27] 409 |                 let orig: Vec<_> = rng.sample_iter::<i32>(&Standard)
[01:00:27]     |                                        ^^^^^^^^^^^ expected 2 type parameters
[01:00:31] error: aborting due to previous error
[01:00:31] 
[01:00:31] For more information about this error, try `rustc --explain E0089`.
[01:00:31] error: Could not compile `alloc`.
[01:00:31] error: Could not compile `alloc`.
[01:00:31] warning: build failed, waiting for other jobs to finish...
[01:00:34] error: build failed
[01:00:34] 
[01:00:34] 
[01:00:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:00:34] 
[01:00:34] 
[01:00:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:34] Build completed unsuccessfully in 0:17:09
[01:00:34] Build completed unsuccessfully in 0:17:09
[01:00:34] make: *** [check] Error 1
[01:00:34] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:284e6890
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
