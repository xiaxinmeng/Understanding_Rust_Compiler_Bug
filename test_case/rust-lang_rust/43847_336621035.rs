
> [00:56:33] ---- [compile-fail] compile-fail-fulldeps/macro-spans.rs stdout ----
> [00:56:33] 	
> [00:56:33] error: /checkout/src/test/compile-fail-fulldeps/macro-spans.rs:16: expected error not found: recursive type `Name::Name` has infinite size
> [00:56:33] 
> [00:56:33] error: 0 unexpected errors found, 1 expected errors not found
> [00:56:33] status: exit code: 101
> [00:56:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail-fulldeps/macro-spans.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/macro-spans.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/macro-spans.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
> [00:56:33] not found errors (from test file): [
> [00:56:33]     Error {
> [00:56:33]         line_num: 16,
> [00:56:33]         kind: Some(
> [00:56:33]             Error
> [00:56:33]         ),
> [00:56:33]         msg: "recursive type `Name::Name` has infinite size"
> [00:56:33]     }
> [00:56:33] ]
> [00:56:33] 
> [00:56:33] thread '[compile-fail] compile-fail-fulldeps/macro-spans.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1084:12
> [00:56:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
> [00:56:33] 
> [00:56:33] 
> [00:56:33] failures:
> [00:56:33]     [compile-fail] compile-fail-fulldeps/macro-spans.rs
> [00:56:33] 
> [00:56:33] test result: FAILED. 52 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
> 