
[01:06:34] ---- [compile-fail] compile-fail/borrowck/two-phase-nonrecv-autoref.rs stdout ----
[01:06:34] 	
[01:06:34] error in revision `nll`: /checkout/src/test/compile-fail/borrowck/two-phase-nonrecv-autoref.rs:102: expected error not found: cannot move a value of type
[01:06:34] 
[01:06:34] error in revision `nll`: /checkout/src/test/compile-fail/borrowck/two-phase-nonrecv-autoref.rs:102: expected error not found: cannot move a value of type
[01:06:34] 
[01:06:34] error in revision `nll`: 0 unexpected errors found, 2 expected errors not found
[01:06:34] status: exit code: 101
[01:06:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/borrowck/two-phase-nonrecv-autoref.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/two-phase-nonrecv-autoref.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-Z" "nll" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/two-phase-nonrecv-autoref.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:06:34] not found errors (from test file): [
[01:06:34]     Error {
[01:06:34]         line_num: 102,
[01:06:34]         kind: Some(
[01:06:34]             Error
[01:06:34]         ),
[01:06:34]         msg: "cannot move a value of type"
[01:06:34]     },
[01:06:34]     Error {
[01:06:34]         line_num: 102,
[01:06:34]         kind: Some(
[01:06:34]             Error
[01:06:34]         ),
[01:06:34]         msg: "cannot move a value of type"
[01:06:34]     }
[01:06:34] ]
[01:06:34] 
[01:06:34] thread '[compile-fail] compile-fail/borrowck/two-phase-nonrecv-autoref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1253:13
[01:06:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:34] 
[01:06:34] 
[01:06:34] failures:
[01:06:34]     [compile-fail] compile-fail/borrowck/two-phase-nonrecv-autoref.rs
