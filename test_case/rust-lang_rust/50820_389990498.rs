
[01:13:54] ---- [run-pass] run-pass-fulldeps/macro-quote-test.rs stdout ----
[01:13:54] 	
[01:13:54] error: compilation failed!
[01:13:54] status: exit code: 101
[01:13:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/macro-quote-test.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-test/auxiliary"
[01:13:54] stdout:
[01:13:54] ------------------------------------------
[01:13:54] 
[01:13:54] ------------------------------------------
[01:13:54] stderr:
[01:13:54] ------------------------------------------
[01:13:54] error[E0658]: procedural macros cannot expand to macro definitions
[01:13:54]   --> /checkout/src/test/run-pass-fulldeps/macro-quote-test.rs:21:5
[01:13:54]    |
[01:13:54] 21 |     hello_macro::hello!();
[01:13:54]    |     ^^^^^^^^^^^^^^^^^^^^^^
[01:13:54]    |
[01:13:54]    = help: add #![feature(proc_macro_gen)] to the crate attributes to enable
[01:13:54] 
[01:13:54] error: aborting due to previous error
[01:13:54] 
[01:13:54] For more information about this error, try `rustc --explain E0658`.
[01:13:54] 
[01:13:54] ------------------------------------------
[01:13:54] 
[01:13:54] thread '[run-pass] run-pass-fulldeps/macro-quote-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[01:13:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:54] 
[01:13:54] 
[01:13:54] failures:
[01:13:54]     [run-pass] run-pass-fulldeps/macro-quote-test.rs
[01:13:54] 
[01:13:54] test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
