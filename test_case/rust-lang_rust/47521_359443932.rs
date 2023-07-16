
[00:54:16] ---- [run-pass] run-pass/thinlto/all-crates.rs stdout ----
[00:54:16] 	
[00:54:16] error: compilation failed!
[00:54:16] status: exit code: 1
[00:54:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/thinlto/all-crates.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thinlto/all-crates.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thinlto/all-crates.stage2-x86_64-unknown-linux-gnu.aux"
[00:54:16] stdout:
[00:54:16] ------------------------------------------
[00:54:16] 
[00:54:16] ------------------------------------------
[00:54:16] stderr:
[00:54:16] ------------------------------------------
[00:54:16] LLVM ERROR: ThinLTO not available
[00:54:16] 
[00:54:16] ------------------------------------------
[00:54:16] 
[00:54:16] thread '[run-pass] run-pass/thinlto/all-crates.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2884:9
[00:54:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:16] 
[00:54:16] 
[00:54:16] failures:
[00:54:16]     [run-pass] run-pass/thinlto/all-crates.rs
