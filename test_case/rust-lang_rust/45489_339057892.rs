
[00:55:21] ---- [compile-fail] compile-fail/issue-27842.rs stdout ----
[00:55:21]
[00:55:21] error: /checkout/src/test/compile-fail/issue-27842.rs:14: expected suggestion not found: let _ = tup.0
[00:55:21] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329:21
[00:55:21]
[00:55:21] error: 0 unexpected errors found, 1 expected errors not found
[00:55:21] status: exit code: 101
[00:55:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-27842.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-27842.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-27842.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:55:21] not found errors (from test file): [
[00:55:21] Error {
[00:55:21] line_num: 14,
[00:55:21] kind: Some(
[00:55:21] Suggestion
[00:55:21] ),
[00:55:21] msg: "let _ = tup.0"
[00:55:21] }
[00:55:21] ]
[00:55:21]
[00:55:21] thread '[compile-fail] compile-fail/issue-27842.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1085:12
[00:55:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:21]
[00:55:21] ---- [compile-fail] compile-fail/missing-block-hint.rs stdout ----
[00:55:21]
[00:55:21] error: /checkout/src/test/compile-fail/missing-block-hint.rs:17: expected suggestion not found: { bar; }
[00:55:21]
[00:55:21] error: 0 unexpected errors found, 1 expected errors not found
[00:55:21] status: exit code: 101
[00:55:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/missing-block-hint.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/missing-block-hint.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/missing-block-hint.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:55:21] not found errors (from test file): [
[00:55:21] Error {
[00:55:21] line_num: 17,
[00:55:21] kind: Some(
[00:55:21] Suggestion
[00:55:21] ),
[00:55:21] msg: "{ bar; }"
[00:55:21] }
[00:55:21] ]
[00:55:21]
[00:55:21] thread '[compile-fail] compile-fail/missing-block-hint.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1085:12
[00:55:21]
[00:55:21]
[00:55:21] failures:
[00:55:21] [compile-fail] compile-fail/issue-27842.rs
[00:55:21] [compile-fail] compile-fail/missing-block-hint.rs
[00:55:21]
[00:55:21] test result: FAILED. 2765 passed; 2 failed; 13 ignored; 0 measured; 0 filtered out
