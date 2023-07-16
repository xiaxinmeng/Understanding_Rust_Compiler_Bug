
[00:43:50] ---- [compile-fail] compile-fail/asm-src-loc-codegen-units.rs stdout ----
[00:43:50] 	
[00:43:50] error: compile-fail test compiled successfully!
[00:43:50] status: exit code: 0
[00:43:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/asm-src-loc-codegen-units.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "codegen-units=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:43:50] stdout:
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] ------------------------------------------
[00:43:50] stderr:
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] thread '[compile-fail] compile-fail/asm-src-loc-codegen-units.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:43:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:50] 
[00:43:50] ---- [compile-fail] compile-fail/asm-src-loc.rs stdout ----
[00:43:50] 	
[00:43:50] error: compile-fail test compiled successfully!
[00:43:50] status: exit code: 0
[00:43:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/asm-src-loc.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:43:50] stdout:
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] ------------------------------------------
[00:43:50] stderr:
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] ------------------------------------------
[00:43:50] 
[00:43:50] thread '[compile-fail] compile-fail/asm-src-loc.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:43:50] 
[00:43:50] 
[00:43:50] failures:
[00:43:50]     [compile-fail] compile-fail/asm-src-loc-codegen-units.rs
[00:43:50]     [compile-fail] compile-fail/asm-src-loc.rs
[00:43:50] 
[00:43:50] test result: FAILED. 2748 passed; 2 failed; 13 ignored; 0 measured; 0 filtered out
