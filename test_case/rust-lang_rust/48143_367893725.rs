
[01:07:29] ---- [compile-fail] compile-fail/issue-12997-2.rs stdout ----
[01:07:29] 	
[01:07:29] error: /checkout/src/test/compile-fail/issue-12997-2.rs:16: expected message not found: expected type `for<'r> fn(&'r mut __test::test::Bencher)`
[01:07:29] 
[01:07:29] error: /checkout/src/test/compile-fail/issue-12997-2.rs:16: expected message not found: found type `fn(isize) {bar}`
[01:07:29] 
[01:07:29] error: /checkout/src/test/compile-fail/issue-12997-2.rs:16: expected message not found: expected mutable reference, found isize
[01:07:29] 
[01:07:29] error: 0 unexpected errors found, 3 expected errors not found
[01:07:29] status: exit code: 101
[01:07:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-12997-2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-12997-2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-12997-2.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:07:29] not found errors (from test file): [
[01:07:29]     Error {
[01:07:29]         line_num: 16,
[01:07:29]         kind: None,
[01:07:29]         msg: "expected type `for<\'r> fn(&\'r mut __test::test::Bencher)`"
[01:07:29]     },
[01:07:29]     Error {
[01:07:29]         line_num: 16,
[01:07:29]         kind: None,
[01:07:29]         msg: "found type `fn(isize) {bar}`"
[01:07:29]     },
[01:07:29]     Error {
[01:07:29]         line_num: 16,
[01:07:29]         kind: None,
[01:07:29]         msg: "expected mutable reference, found isize"
[01:07:29]     }
[01:07:29] ]
[01:07:29] 
[01:07:29] thread '[compile-fail] compile-fail/issue-12997-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1253:13
[01:07:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:29] 
[01:07:29] 
[01:07:29] failures:
[01:07:29]     [compile-fail] compile-fail/issue-12997-2.rs
[01:07:29] 
[01:07:29] test result: FAILED. 2303 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
[01:07:29] 
[01:07:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
