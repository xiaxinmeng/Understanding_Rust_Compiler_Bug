
[00:40:48] failures:
[00:40:48] 
[00:40:48] ---- [compile-fail] compile-fail/union/union-lint-dead-code.rs stdout ----
[00:40:48] 	
[00:40:48] error: compile-fail test compiled successfully!
[00:40:48] status: exit code: 0
[00:40:48] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/union/union-lint-dead-code.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/union/union-lint-dead-code.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/union/union-lint-dead-code.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:40:48] stdout:
[00:40:48] ------------------------------------------
[00:40:48] 
[00:40:48] ------------------------------------------
[00:40:48] stderr:
[00:40:48] ------------------------------------------
[00:40:48] 
[00:40:48] ------------------------------------------
[00:40:48] 
[00:40:48] thread '[compile-fail] compile-fail/union/union-lint-dead-code.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2479:8
[00:40:48] 
[00:40:48] 
[00:40:48] failures:
[00:40:48]     [compile-fail] compile-fail/union/union-lint-dead-code.rs
[00:40:48] 
[00:40:48] test result: FAILED. 2704 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
