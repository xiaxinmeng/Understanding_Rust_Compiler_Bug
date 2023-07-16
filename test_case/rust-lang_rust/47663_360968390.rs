
[00:56:16] ---- [ui] ui/test-should-panic-attr.rs stdout ----
[00:56:16] 	
[00:56:16] error: test run failed!
[00:56:16] status: signal: 6
[00:56:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr.stage2-i686-unknown-linux-musl"
[00:56:16] stdout:
[00:56:16] ------------------------------------------
[00:56:16] 
[00:56:16] running 5 tests
[00:56:16] 
[00:56:16] ------------------------------------------
[00:56:16] stderr:
[00:56:16] ------------------------------------------
[00:56:16] fatal runtime error: failed to initiate panic, error 5
[00:56:16] 
[00:56:16] ------------------------------------------
[00:56:16] 
[00:56:16] thread '[ui] ui/test-should-panic-attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[00:56:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:16] 
[00:56:16] 
[00:56:16] failures:
[00:56:16]     [ui] ui/test-should-panic-attr.rs
