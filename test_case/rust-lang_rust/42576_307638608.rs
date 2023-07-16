
00:39:40] ---- [compile-fail] compile-fail/dep-graph-unrelated.rs stdout ----
[00:39:40] 	
[00:39:40] error: /checkout/src/test/compile-fail/dep-graph-unrelated.rs:21: unexpected "error": '21:1: 21:46: unrecognized DepNode variant TransCrateItem(88)'
[00:39:40] 
[00:39:40] error: /checkout/src/test/compile-fail/dep-graph-unrelated.rs:21: expected error not found: no path from `main`
[00:39:40] 
[00:39:40] error: 1 unexpected errors found, 1 expected errors not found
[00:39:40] status: exit code: 101
[00:39:40] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/dep-graph-unrelated.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dep-graph-unrelated.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/dep-graph-unrelated.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers -Z query-dep-graph
[00:39:40] unexpected errors (from JSON output): [
[00:39:40]     Error {
[00:39:40]         line_num: 21,
[00:39:40]         kind: Some(
[00:39:40]             Error
[00:39:40]         ),
[00:39:40]         msg: "21:1: 21:46: unrecognized DepNode variant TransCrateItem(88)"
[00:39:40]     }
[00:39:40] ]
[00:39:40] 
[00:39:40] not found errors (from test file): [
[00:39:40]     Error {
[00:39:40]         line_num: 21,
[00:39:40]         kind: Some(
[00:39:40]             Error
[00:39:40]         ),
[00:39:40]         msg: "no path from `main`"
[00:39:40]     }
[00:39:40] ]
[00:39:40] 
[00:39:40] thread '[compile-fail] compile-fail/dep-graph-unrelated.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1129
[00:39:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
