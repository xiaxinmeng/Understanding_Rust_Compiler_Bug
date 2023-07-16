
[00:55:34] ---- [compile-fail] compile-fail/associated-const-impl-wrong-lifetime.rs stdout ----

[00:55:34] 	

[00:55:34] error: /checkout/src/test/compile-fail/associated-const-impl-wrong-lifetime.rs:19: unexpected note: '19:5: 19:34: expected type `&str`'

[00:55:34] 

[00:55:34] error: /checkout/src/test/compile-fail/associated-const-impl-wrong-lifetime.rs:19: expected note not found: expected type `&'static str`

[00:55:34] 

[00:55:34] error: 1 unexpected errors found, 1 expected errors not found

[00:55:34] status: exit code: 101

[00:55:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/associated-const-impl-wrong-lifetime.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-const-impl-wrong-lifetime.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-const-impl-wrong-lifetime.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"

[00:55:34] unexpected errors (from JSON output): [

[00:55:34]     Error {

[00:55:34]         line_num: 19,

[00:55:34]         kind: Some(

[00:55:34]             Note

[00:55:34]         ),

[00:55:34]         msg: "19:5: 19:34: expected type `&str`"

[00:55:34]     }

[00:55:34] ]

[00:55:34] 

[00:55:34] not found errors (from test file): [

[00:55:34]     Error {

[00:55:34]         line_num: 19,

[00:55:34]         kind: Some(

[00:55:34]             Note

[00:55:34]         ),

[00:55:34]         msg: "expected type `&\'static str`"

[00:55:34]     }

[00:55:34] ]

[00:55:34] 

[00:55:34] thread '[compile-fail] compile-fail/associated-const-impl-wrong-lifetime.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1086:12

[00:55:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[00:55:34] 

[00:55:34] ---- [compile-fail] compile-fail/regions-infer-paramd-indirect.rs stdout ----
