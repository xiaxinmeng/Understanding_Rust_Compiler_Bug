
[00:50:04] ---- [compile-fail] compile-fail/simd-intrinsic-generic-arithmetic.rs stdout ----
[00:50:04] 	
[00:50:04] error: /checkout/src/test/compile-fail/simd-intrinsic-generic-arithmetic.rs:87: expected error not found: unsupported operation on `i32x4` with element `i32`
[00:50:04] 
[00:50:04] error: /checkout/src/test/compile-fail/simd-intrinsic-generic-arithmetic.rs:89: expected error not found: unsupported operation on `u32x4` with element `u32`
[00:50:04] 
[00:50:04] error: 0 unexpected errors found, 2 expected errors not found
[00:50:04] status: exit code: 101
[00:50:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/simd-intrinsic-generic-arithmetic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/simd-intrinsic-generic-arithmetic.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/simd-intrinsic-generic-arithmetic.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:50:04] not found errors (from test file): [
[00:50:04]     Error {
[00:50:04]         line_num: 87,
[00:50:04]         kind: Some(
[00:50:04]             Error
[00:50:04]         ),
[00:50:04]         msg: "unsupported operation on `i32x4` with element `i32`"
[00:50:04]     },
[00:50:04]     Error {
[00:50:04]         line_num: 89,
[00:50:04]         kind: Some(
[00:50:04]             Error
[00:50:04]         ),
[00:50:04]         msg: "unsupported operation on `u32x4` with element `u32`"
[00:50:04]     }
[00:50:04] ]
[00:50:04] 
[00:50:04] thread '[compile-fail] compile-fail/simd-intrinsic-generic-arithmetic.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1085:12
[00:50:04] 
[00:50:04] 
[00:50:04] failures:
[00:50:04]     [compile-fail] compile-fail/simd-intrinsic-generic-arithmetic.rs
