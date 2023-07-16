plain
[00:42:38] .......................................................................i............................
[00:42:42] ....................................................................................................
[00:42:47] ....................................................................................................
[00:42:53] ....................................................................................................
[00:42:56] i.................iiiiiiiii...................................................
[00:42:56] 
[00:42:56] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:43:43] .......................................................................i............................
[00:43:47] ....................................................................................................
[00:43:51] ....................................................................................................
[00:43:57] ....................................................................................................
[00:44:00] i.................iiiiiiiii...................................................
[00:44:00] 
[00:44:00]  finished in 63.489
[00:44:00] travis_fold:end:test_ui_nll

---
[00:54:02] failures:
[00:54:02] 
[00:54:02] ---- [compile-fail] compile-fail/issue-43431.rs stdout ----
[00:54:02] 
[00:54:02] error: /checkout/src/test/compile-fail/issue-43431.rs:14: unexpected warning: '14:5: 14:54: the trait `CallSingle` cannot be made into an object [where_clauses_object_safety]'
[00:54:02] 
[00:54:02] error: /checkout/src/test/compile-fail/issue-43431.rs:14: unexpected warning: '14:5: 14:54: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
[00:54:02] 
[00:54:02] error: 2 unexpected errors found, 0 expected errors not found
[00:54:02] status: exit code: 101
[00:54:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-43431.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43431/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43431/auxiliary" "-A" "unused"
[00:54:02] unexpected errors (from JSON output): [
[00:54:02]     Error {
[00:54:02]         line_num: 14,
[00:54:02]         kind: Some(
[00:54:02]         ),
[00:54:02]         ),
[00:54:02]         msg: "14:5: 14:54: the trait `CallSingle` cannot be made into an object [where_clauses_object_safety]"
[00:54:02]     Error {
[00:54:02]         line_num: 14,
[00:54:02]         kind: Some(
[00:54:02]             Warning
[00:54:02]             Warning
[00:54:02]         ),
[00:54:02]         msg: "14:5: 14:54: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!"
[00:54:02] ]
[00:54:02] 
[00:54:02] thread '[compile-fail] compile-fail/issue-43431.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:54:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:02] 
[00:54:02] ---- [compile-fail] compile-fail/wf-trait-fn-where-clause.rs stdout ----
[00:54:02] 
[00:54:02] error: /checkout/src/test/compile-fail/wf-trait-fn-where-clause.rs:20: unexpected warning: '20:5: 20:41: the trait `Foo` cannot be made into an object [where_clauses_object_safety]'
[00:54:02] 
[00:54:02] error: /checkout/src/test/compile-fail/wf-trait-fn-where-clause.rs:20: unexpected warning: '20:5: 20:41: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
[00:54:02] 
[00:54:02] error: 2 unexpected errors found, 0 expected errors not found
[00:54:02] status: exit code: 101
[00:54:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/wf-trait-fn-where-clause.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/wf-trait-fn-where-clause/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/wf-trait-fn-where-clause/auxiliary" "-A" "unused"
[00:54:02] unexpected errors (from JSON output): [
[00:54:02]     Error {
[00:54:02]         line_num: 20,
[00:54:02]         kind: Some(
[00:54:02]         ),
[00:54:02]         ),
[00:54:02]         msg: "20:5: 20:41: the trait `Foo` cannot be made into an object [where_clauses_object_safety]"
[00:54:02]     Error {
[00:54:02]         line_num: 20,
[00:54:02]         kind: Some(
[00:54:02]             Warning
[00:54:02]             Warning
[00:54:02]         ),
[00:54:02]         msg: "20:5: 20:41: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!"
[00:54:02] ]
[00:54:02] 
[00:54:02] thread '[compile-fail] compile-fail/wf-trait-fn-where-clause.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:54:02] 
