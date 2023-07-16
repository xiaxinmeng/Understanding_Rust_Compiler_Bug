
[00:59:35] 
[00:59:35] ---- [compile-fail] compile-fail/nll/where_clauses_in_repeat_rvalue.rs stdout ----
[00:59:35] 	
[00:59:35] error: /checkout/src/test/compile-fail/nll/where_clauses_in_repeat_rvalue.rs:30: unexpected error: '30:17: 30:19: `x` does not live long enough [E0597]'
[00:59:35] 
[00:59:35] error: /checkout/src/test/compile-fail/nll/where_clauses_in_repeat_rvalue.rs:38: expected error not found: borrowed value does not live long enough [E0597]
[00:59:35] 
[00:59:35] error: 1 unexpected errors found, 1 expected errors not found
[00:59:35] status: exit code: 101
[00:59:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/nll/where_clauses_in_repeat_rvalue.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/nll/where_clauses_in_repeat_rvalue.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "nll" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/nll/where_clauses_in_repeat_rvalue.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:35] unexpected errors (from JSON output): [
[00:59:35]     Error {
[00:59:35]         line_num: 30,
[00:59:35]         kind: Some(
[00:59:35]             Error
[00:59:35]         ),
[00:59:35]         msg: "30:17: 30:19: `x` does not live long enough [E0597]"
[00:59:35]     }
[00:59:35] ]
[00:59:35] 
[00:59:35] not found errors (from test file): [
[00:59:35]     Error {
[00:59:35]         line_num: 38,
[00:59:35]         kind: Some(
[00:59:35]             Error
[00:59:35]         ),
[00:59:35]         msg: "borrowed value does not live long enough [E0597]"
[00:59:35]     }
[00:59:35] ]
[00:59:35] 
[00:59:35] thread '[compile-fail] compile-fail/nll/where_clauses_in_repeat_rvalue.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1163:12
[00:59:35] 
[00:59:35] 
[00:59:35] failures:
[00:59:35]     [compile-fail] compile-fail/nll/where_clauses_in_repeat_rvalue.rs
[00:59:35] 
[00:59:35] test result: [31mFAILED(B[m. 2698 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
[00:59:35] 
