
[00:39:59] ---- [compile-fail] compile-fail/proc-macro/shadow.rs stdout ----
[00:39:59] 	
[00:39:59] error: /checkout/src/test/compile-fail-fulldeps/proc-macro/shadow.rs:16: unexpected "error": '16:1: 16:23: the name `derive_a` is defined multiple times [E0259]'
[00:39:59] 
[00:39:59] error: /checkout/src/test/compile-fail-fulldeps/proc-macro/shadow.rs:16: expected error not found: the name `derive_a` is defined twice
[00:39:59] 
[00:39:59] error: 1 unexpected errors found, 1 expected errors not found
[00:39:59] status: exit code: 101
[00:39:59] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail-fulldeps/proc-macro/shadow.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/proc-macro/shadow.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/proc-macro/shadow.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:39:59] unexpected errors (from JSON output): [
[00:39:59]     Error {
[00:39:59]         line_num: 16,
[00:39:59]         kind: Some(
[00:39:59]             Error
[00:39:59]         ),
[00:39:59]         msg: "16:1: 16:23: the name `derive_a` is defined multiple times [E0259]"
[00:39:59]     }
[00:39:59] ]
[00:39:59] 
[00:39:59] not found errors (from test file): [
[00:39:59]     Error {
[00:39:59]         line_num: 16,
[00:39:59]         kind: Some(
[00:39:59]             Error
[00:39:59]         ),
[00:39:59]         msg: "the name `derive_a` is defined twice"
[00:39:59]     }
[00:39:59] ]
[00:39:59] 
[00:39:59] thread '[compile-fail] compile-fail/proc-macro/shadow.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1129
[00:39:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
