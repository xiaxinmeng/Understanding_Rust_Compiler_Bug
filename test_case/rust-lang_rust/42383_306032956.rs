
[00:47:37] error: /checkout/src/test/compile-fail/method-call-err-msg.rs:34: unexpected "note": '34:7: 34:11: the following traits define an item `take`, perhaps you need to implement one of them:'

[00:47:37] 

[00:47:37] error: 1 unexpected errors found, 0 expected errors not found

[00:47:37] status: exit code: 101

[00:47:37] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/method-call-err-msg.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-err-msg.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-err-msg.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers

[00:47:37] unexpected errors (from JSON output): [

[00:47:37]     Error {

[00:47:37]         line_num: 34,

[00:47:37]         kind: Some(

[00:47:37]             Note

[00:47:37]         ),

[00:47:37]         msg: "34:7: 34:11: the following traits define an item `take`, perhaps you need to implement one of them:"

[00:47:37]     }

[00:47:37] ]

[00:47:37] 

[00:47:37] thread '[compile-fail] compile-fail/method-call-err-msg.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1129

[00:47:37] 

[00:47:37] 

[00:47:37] failures:

[00:47:37]     [compile-fail] compile-fail/method-call-err-msg.rs

[00:47:37] 

[00:47:37] test result: FAILED. 2653 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
