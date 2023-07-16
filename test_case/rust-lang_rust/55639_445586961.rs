

[00:56:06] failures:
[00:56:06] 
[00:56:06] ---- [ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let.rs stdout ----
[00:56:06] 
[00:56:06] error: /checkout/src/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let.rs:13: unexpected error: '13:5: 15:6: irrefutable if-let pattern [irrefutable_let_patterns]'
[00:56:06] 
[00:56:06] error: 1 unexpected errors found, 0 expected errors not found
[00:56:06] status: exit code: 1
[00:56:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let/auxiliary" "-A" "unused"
[00:56:06] unexpected errors (from JSON output): [
[00:56:06]     Error {
[00:56:06]         line_num: 13,
[00:56:06]         kind: Some(
[00:56:06]             Error
[00:56:06]         ),
[00:56:06]         msg: "13:5: 15:6: irrefutable if-let pattern [irrefutable_let_patterns]"
[00:56:06]     }
[00:56:06] ]
[00:56:06] 
[00:56:06] thread '[ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:56:06] 
[00:56:06] ---- [ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let.rs stdout ----
[00:56:06] 
[00:56:06] error: /checkout/src/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let.rs:13: unexpected error: '13:5: 15:6: irrefutable while-let pattern [irrefutable_let_patterns]'
[00:56:06] 
[00:56:06] error: 1 unexpected errors found, 0 expected errors not found
[00:56:06] status: exit code: 1
[00:56:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let/auxiliary" "-A" "unused"
[00:56:06] unexpected errors (from JSON output): [
[00:56:06]     Error {
[00:56:06]         line_num: 13,
[00:56:06]         kind: Some(
[00:56:06]             Error
[00:56:06]         ),
[00:56:06]         msg: "13:5: 15:6: irrefutable while-let pattern [irrefutable_let_patterns]"
[00:56:06]     }
[00:56:06] ]
[00:56:06] 
[00:56:06] thread '[ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:56:06] 
[00:56:06] 
[00:56:06] failures:
[00:56:06]     [ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-if-let.rs
[00:56:06]     [ui] ui/rfc-2086-irrefutable_let_patterns/deny-irrefutable-while-let.rs
[00:56:06] 
[00:56:06] test result: FAILED. 5081 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
