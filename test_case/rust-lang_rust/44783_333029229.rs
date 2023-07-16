
[00:57:29] ---- [run-pass] run-pass/lto-many-codegen-units.rs stdout ----
[00:57:29] 	
[00:57:29] error: compilation failed!
[00:57:29] status: signal: 11
[00:57:29] command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/lto-many-codegen-units.rs" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-pass" "--target=i686-unknown-linux-gnu" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-pass/lto-many-codegen-units.stage2-i686-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "codegen-units=8" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-pass/lto-many-codegen-units.stage2-i686-unknown-linux-gnu.run-pass.libaux"
[00:57:29] stdout:
[00:57:29] ------------------------------------------
[00:57:29] 
[00:57:29] ------------------------------------------
[00:57:29] stderr:
[00:57:29] ------------------------------------------
[00:57:29] 
[00:57:29] ------------------------------------------
[00:57:29] 
[00:57:29] thread '[run-pass] run-pass/lto-many-codegen-units.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:57:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:29] 
...
[00:57:29] 
[00:57:29] 
[00:57:29] failures:
[00:57:29]     [run-pass] run-pass/lto-many-codegen-units.rs
[00:57:29]     [run-pass] run-pass/panic-runtime/lto-abort.rs
[00:57:29]     [run-pass] run-pass/panic-runtime/lto-unwind.rs
[00:57:29]     [run-pass] run-pass/sepcomp-lib-lto.rs
[00:57:29]     [run-pass] run-pass/stack-probes-lto.rs
[00:57:29] 
[00:57:29] test result: FAILED. 2763 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out
