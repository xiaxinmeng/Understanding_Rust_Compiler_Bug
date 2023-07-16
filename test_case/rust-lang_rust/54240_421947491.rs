plain
[00:52:24] ....................................................................................................
[00:52:27] .....................................................i..............................................
[00:52:29] ....................................................................................................
[00:52:32] ....................................................................................................
[00:52:35] .iiiiiiiii..........................................................................................
[00:52:41] ....................................................................................................
[00:52:44] ..................................................................................i.................
[00:52:47] ....................................................................................................
[00:52:50] ....................................i.i..ii.........................................................
---
[01:07:42]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:07:50] error[E0282]: type annotations needed
[01:07:50]    --> libcore/../libcore/tests/nonzero.rs:128:16
[01:07:50]     |
[01:07:50] 128 |     assert_eq!(nz.into(), 1u32);
[01:07:50]     |                ^^^^^^^^^ cannot infer type for `T`
[01:07:56] error: aborting due to previous error
[01:07:56] 
[01:07:56] For more information about this error, try `rustc --explain E0282`.
[01:07:56] error: Could not compile `core`.
[01:07:56] error: Could not compile `core`.
[01:07:56] 
[01:07:56] To learn more, run the command again with --verbose.
[01:07:56] 
[01:07:56] 
[01:07:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:07:56] 
[01:07:56] 
[01:07:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:56] Build completed unsuccessfully in 0:23:56
[01:07:56] Build completed unsuccessfully in 0:23:56
[01:07:56] Makefile:58: recipe for target 'check' failed
[01:07:56] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12e876e2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
