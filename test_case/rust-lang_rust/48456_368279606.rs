
[00:05:02] * 534 error codes
[00:05:02] * highest error code: E0908
[00:05:02] * 175 features
[00:05:02] Getting metadata from "/checkout/src/Cargo.toml"
[00:05:02] thread 'main' panicked at 'Unable to run `cargo metadata`: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5
[00:05:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:02] 
[00:05:02] 
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "--no-vendor" "--quiet"
[00:05:02] expected success, got: exit code: 101
[00:05:02] 
[00:05:02] 
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:01:57
[00:05:02] Makefile:74: recipe for target 'tidy' failed
[00:05:02] make: *** [tidy] Error 1
