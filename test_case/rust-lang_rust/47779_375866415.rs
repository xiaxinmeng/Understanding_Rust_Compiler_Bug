
[01:05:58] failures:
[01:05:58] 
[01:05:58] ---- [run-pass] run-pass/issue-33992.rs stdout ----
[01:05:58] 	
[01:05:58] error: compilation failed!
[01:05:58] status: signal: 6
[01:05:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-33992.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=wasm32-unknown-unknown" "-Ccodegen-units=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33992.stage2-wasm32-unknown-unknown.wasm" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33992.stage2-wasm32-unknown-unknown.aux"
[01:05:58] stdout:
[01:05:58] ------------------------------------------
[01:05:58] 
[01:05:58] ------------------------------------------
[01:05:58] stderr:
[01:05:58] ------------------------------------------
[01:05:58] Common symbols are not yet implemented for Wasm
[01:05:58] UNREACHABLE executed at /checkout/src/llvm/lib/MC/MCWasmStreamer.cpp:134!
[01:05:58] 
[01:05:58] ------------------------------------------
[01:05:58] 
[01:05:58] thread '[run-pass] run-pass/issue-33992.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2918:9
[01:05:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:58] 
[01:05:58] 
[01:05:58] failures:
[01:05:58]     [run-pass] run-pass/issue-33992.rs
