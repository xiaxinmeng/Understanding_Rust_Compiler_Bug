plain
[00:50:38] ............................................................................ii......................
[00:51:25] ........................................i....................................................i.ii...
[00:51:35] ........................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:52:06] ............................................................................
[00:52:26] .iiiiiii............................................................................................
[00:53:05] ....................................................................................................
[00:53:22] .........................................................................
[00:53:22] test result: ok. 2954 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:53:22] 
---
[01:06:21] 
[01:06:21] To learn more, run the command again with --verbose.
[01:06:21] 
[01:06:21] 
[01:06:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:06:21] 
[01:06:21] 
[01:06:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:21] Build completed unsuccessfully in 0:26:23
[01:06:21] Build completed unsuccessfully in 0:26:23
[01:06:21] Makefile:58: recipe for target 'check' failed
[01:06:21] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11912cd8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
