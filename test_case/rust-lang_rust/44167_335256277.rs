
00:56:42] ---- [compile-fail] compile-fail/associated-types/cache/project-fn-ret-invariant.rs stdout ----
[00:56:42] 	
[00:56:42] error in revision `krisskross`: /checkout/src/test/compile-fail/associated-types/cache/project-fn-ret-invariant.rs:66: unexpected "error": '66:5: 66:6: lifetime mismatch [E0623]'
[00:56:42] 
[00:56:42] error in revision `krisskross`: /checkout/src/test/compile-fail/associated-types/cache/project-fn-ret-invariant.rs:64: expected error not found: 64:21: 64:22: lifetime mismatch [E0623]
[00:56:42] 
[00:56:42] error in revision `krisskross`: 1 unexpected errors found, 1 expected errors not found
[00:56:42] status: exit code: 101
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/associated-types/cache/project-fn-ret-invariant.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-types/cache/project-fn-ret-invariant.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/associated-types/cache/project-fn-ret-invariant.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:56:42] unexpected errors (from JSON output): [
[00:56:42]     Error {
[00:56:42]         line_num: 66,
[00:56:42]         kind: Some(
[00:56:42]             Error
[00:56:42]         ),
[00:56:42]         msg: "66:5: 66:6: lifetime mismatch [E0623]"
[00:56:42]     }
[00:56:42] ]
[00:56:42] 
[00:56:42] not found errors (from test file): [
[00:56:42]     Error {
[00:56:42]         line_num: 64,
[00:56:42]         kind: Some(
[00:56:42]             Error
[00:56:42]         ),
[00:56:42]         msg: "64:21: 64:22: lifetime mismatch [E0623]"
[00:56:42]     }
[00:56:42] ]
[00:56:42] 
[00:56:42] thread '[compile-fail] compile-fail/associated-types/cache/project-fn-ret-invariant.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1094:12
[00:56:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
