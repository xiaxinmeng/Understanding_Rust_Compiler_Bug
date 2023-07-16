
[00:49:18] error: error pattern ' dependency `cdylib_dep` not found in rlib format' not found!
[00:49:18] status: exit code: 101
[00:49:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/cdylib-deps-must-be-static.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cdylib-deps-must-be-static.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/cdylib-deps-must-be-static.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:49:18] stdout:
[00:49:18] ------------------------------------------
[00:49:18] 
[00:49:18] ------------------------------------------
[00:49:18] stderr:
[00:49:18] ------------------------------------------
[00:49:18] warning: unused extern crate
[00:49:18]   --> /checkout/src/test/compile-fail/cdylib-deps-must-be-static.rs:18:1
[00:49:18]    |
[00:49:18] 18 | extern crate cdylib_dep;
[00:49:18]    | ^^^^^^^^^^^^^^^^^^^^^^^^
[00:49:18]    |
[00:49:18]    = note: #[warn(unused_extern_crates)] on by default
[00:49:18] 
[00:49:18] error: crate `cdylib_dep` required to be available in rlib format, but it was not available in this form
[00:49:18] 
[00:49:18] error: aborting due to previous error
[00:49:18] 
[00:49:18] 
[00:49:18] ------------------------------------------
[00:49:18] 
[00:49:18] thread '[compile-fail] compile-fail/cdylib-deps-must-be-static.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:49:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:18] 
[00:49:18] 
[00:49:18] failures:
[00:49:18]     [compile-fail] compile-fail/cdylib-deps-must-be-static.rs
[00:49:18] 
[00:49:18] test result: [31mFAILED(B[m. 2736 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
[00:49:18] 
[00:49:18] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[00:49:18] 
