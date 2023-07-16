
[00:38:10] ---- [compile-fail] compile-fail/feature-gate-loop-break-value.rs stdout ----

[00:38:10] 	

[00:38:10] error: /checkout/src/test/compile-fail/feature-gate-loop-break-value.rs:13: unexpected "error": '13:15: 13:18: mismatched types [E0308]'

[00:38:10] 

[00:38:10] error: /checkout/src/test/compile-fail/feature-gate-loop-break-value.rs:13: expected error not found: `break` with a value is experimental

[00:38:10] 

[00:38:10] error: 1 unexpected errors found, 1 expected errors not found

[00:38:10] status: exit code: 101

[00:38:10] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/feature-gate-loop-break-value.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-loop-break-value.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-loop-break-value.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers

[00:38:10] unexpected errors (from JSON output): [

[00:38:10]     Error {

[00:38:10]         line_num: 13,

[00:38:10]         kind: Some(

[00:38:10]             Error

[00:38:10]         ),

[00:38:10]         msg: "13:15: 13:18: mismatched types [E0308]"

[00:38:10]     }

[00:38:10] ]

[00:38:10] 

[00:38:10] not found errors (from test file): [

[00:38:10]     Error {

[00:38:10]         line_num: 13,

[00:38:10]         kind: Some(

[00:38:10]             Error

[00:38:10]         ),

[00:38:10]         msg: "`break` with a value is experimental"

[00:38:10]     }

[00:38:10] ]

[00:38:10] 

[00:38:10] thread '[compile-fail] compile-fail/feature-gate-loop-break-value.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1129

[00:38:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
