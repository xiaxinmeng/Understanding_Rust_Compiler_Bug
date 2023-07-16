
[00:54:23] failures:
[00:54:23] 
[00:54:23] ---- [run-pass] run-pass/bind-by-move-with-guard.rs stdout ----
[00:54:23] 	
[00:54:23] error: compilation failed!
[00:54:23] status: exit code: 101
[00:54:23] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/run-pass/bind-by-move-with-guard.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/bind-by-move-with-guard.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/bind-by-move-with-guard.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:54:23] stdout:
[00:54:23] ------------------------------------------
[00:54:23] 
[00:54:23] ------------------------------------------
[00:54:23] stderr:
[00:54:23] ------------------------------------------
[00:54:23] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n