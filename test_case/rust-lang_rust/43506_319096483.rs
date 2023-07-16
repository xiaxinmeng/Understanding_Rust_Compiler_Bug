
[00:49:15] ---- [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.rs stdout ----
[00:49:15] 	
[00:49:15] error: /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.rs:876: expected error not found: compilation successful
[00:49:15] 
[00:49:15] error: 0 unexpected errors found, 1 expected errors not found
[00:49:15] status: exit code: 101
[00:49:15] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:49:15] not found errors (from test file): [
[00:49:15]     Error {
[00:49:15]         line_num: 876,
[00:49:15]         kind: Some(
[00:49:15]             Error
[00:49:15]         ),
[00:49:15]         msg: "compilation successful"
[00:49:15]     }
[00:49:15] ]
[00:49:15] 
[00:49:15] thread '[compile-fail] compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1133:12
[00:49:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:15] 
[00:49:15] 
[00:49:15] failures:
[00:49:15]     [compile-fail] compile-fail/feature-gate/issue-43106-gating-of-builtin-attrs.rs
[00:49:15] 
[00:49:15] test result: FAILED. 2703 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
