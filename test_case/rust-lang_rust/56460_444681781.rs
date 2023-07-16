
[00:50:01] error: /checkout/src/test/ui/nll/issue-55850.rs:38: unexpected error: '38:16: 38:17: `s` does not live long enough [E0597]'
[00:50:01] 
[00:50:01] error: 1 unexpected errors found, 0 expected errors not found
[00:50:01] status: exit code: 1
[00:50:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55850.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/auxiliary" "-A" "unused"
[00:50:01] unexpected errors (from JSON output): [
[00:50:01]     Error {
[00:50:01]         line_num: 38,
[00:50:01]         kind: Some(
[00:50:01]             Error
[00:50:01]         ),
[00:50:01]         msg: "38:16: 38:17: `s` does not live long enough [E0597]"
[00:50:01]     }
[00:50:01] ]
[00:50:01] 
[00:50:01] thread '[ui] ui/nll/issue-55850.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1349:13
