
[00:59:15] failures:
[00:59:15] 
[00:59:15] ---- [compile-fail] compile-fail/dep-graph-variance-alias.rs stdout ----
[00:59:15] 	
[00:59:15] error: /checkout/src/test/compile-fail/dep-graph-variance-alias.rs:29: unexpected error: '29:1: 29:45: no path from `TypeAlias` to `ItemVariances`'
[00:59:15] 
[00:59:15] error: /checkout/src/test/compile-fail/dep-graph-variance-alias.rs:29: expected error not found: OK
[00:59:15] 
[00:59:15] error: 1 unexpected errors found, 1 expected errors not found
[00:59:15] status: exit code: 101
[00:59:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/dep-graph-variance-alias.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dep-graph-variance-alias.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dep-graph-variance-alias.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:15] unexpected errors (from JSON output): [
[00:59:15]     Error {
[00:59:15]         line_num: 29,
[00:59:15]         kind: Some(
[00:59:15]             Error
[00:59:15]         ),
[00:59:15]         msg: "29:1: 29:45: no path from `TypeAlias` to `ItemVariances`"
[00:59:15]     }
[00:59:15] ]
[00:59:15] 
[00:59:15] not found errors (from test file): [
[00:59:15]     Error {
[00:59:15]         line_num: 29,
[00:59:15]         kind: Some(
[00:59:15]             Error
[00:59:15]         ),
[00:59:15]         msg: "OK"
[00:59:15]     }
[00:59:15] ]
[00:59:15] 
[00:59:15] thread '[compile-fail] compile-fail/dep-graph-variance-alias.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1252:13
[00:59:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:15] 
[00:59:15] 
[00:59:15] failures:
[00:59:15]     [compile-fail] compile-fail/dep-graph-variance-alias.rs
[00:59:15] 
[00:59:15] test result: FAILED. 2523 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
