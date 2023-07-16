rust
[00:59:51] ---- [compile-fail] compile-fail/uninhabited-matches-feature-gated.rs stdout ----
[00:59:51] 	
[00:59:51] error: /checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs:25: expected error not found: non-exhaustive
[00:59:51] 
[00:59:51] error: 0 unexpected errors found, 1 expected errors not found
[00:59:51] status: exit code: 101
[00:59:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:51] not found errors (from test file): [
[00:59:51]     Error {
[00:59:51]         line_num: 25,
[00:59:51]         kind: Some(
[00:59:51]             Error
[00:59:51]         ),
[00:59:51]         msg: "non-exhaustive"
[00:59:51]     }
[00:59:51] ]
[00:59:51] 
[00:59:51] thread '[compile-fail] compile-fail/uninhabited-matches-feature-gated.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1170:13
