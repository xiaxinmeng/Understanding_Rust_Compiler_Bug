
[00:35:33] ---- [compile-fail] compile-fail/borrowck/borrowck-in-static.rs stdout ----
[00:35:33] 	
[00:35:33] error: /checkout/src/test/compile-fail/borrowck/borrowck-in-static.rs:15: unexpected "note": 'cannot move out of captured outer variable in an `Fn` closure'
[00:35:33] 
[00:35:33] error: 1 unexpected errors found, 0 expected errors not found
[00:35:33] status: exit code: 101
[00:35:33] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/borrowck/borrowck-in-static.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-in-static.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-in-static.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:35:33] unexpected errors (from JSON output): [
[00:35:33]     Error {
[00:35:33]         line_num: 15,
[00:35:33]         kind: Some(
[00:35:33]             Note
[00:35:33]         ),
[00:35:33]         msg: "cannot move out of captured outer variable in an `Fn` closure"
[00:35:33]     }
[00:35:33] ]
[00:35:33] 
[00:35:33] thread '[compile-fail] compile-fail/borrowck/borrowck-in-static.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1123
[00:35:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:35:33] 
[00:35:33] ---- [compile-fail] compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.rs stdout ----
[00:35:33] 	
[00:35:33] error: /checkout/src/test/compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.rs:21: unexpected "note": 'cannot move out of captured outer variable in an `Fn` closure'
[00:35:33] 
[00:35:33] error: 1 unexpected errors found, 0 expected errors not found
[00:35:33] status: exit code: 101
[00:35:33] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:35:33] unexpected errors (from JSON output): [
[00:35:33]     Error {
[00:35:33]         line_num: 21,
[00:35:33]         kind: Some(
[00:35:33]             Note
[00:35:33]         ),
[00:35:33]         msg: "cannot move out of captured outer variable in an `Fn` closure"
[00:35:33]     }
[00:35:33] ]
[00:35:33] 
[00:35:33] thread '[compile-fail] compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1123
[00:35:33] 
[00:35:33] 
[00:35:33] failures:
[00:35:33]     [compile-fail] compile-fail/borrowck/borrowck-in-static.rs
[00:35:33]     [compile-fail] compile-fail/unboxed-closures-move-upvar-from-non-once-ref-closure.rs
