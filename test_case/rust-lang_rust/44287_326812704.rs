
[00:50:26] ---- [run-pass] run-pass/for-loop-unconstrained-element-type-i32-fallback.rs stdout ----
[00:50:26] 	
[00:50:26] error: compilation failed!
[00:50:26] status: exit code: 101
[00:50:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/for-loop-unconstrained-element-type-i32-fallback.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-unconstrained-element-type-i32-fallback.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/for-loop-unconstrained-element-type-i32-fallback.stage2-x86_64-unknown-linux-gnu.run-pass.libaux"
[00:50:26] stdout:
[00:50:26] ------------------------------------------
[00:50:26] 
[00:50:26] ------------------------------------------
[00:50:26] stderr:
[00:50:26] ------------------------------------------
[00:50:26] error[E0282]: type annotations needed
[00:50:26]   --> /checkout/src/test/run-pass/for-loop-unconstrained-element-type-i32-fallback.rs:17:5
[00:50:26]    |
[00:50:26] 17 |        for i in Vec::new() {
[00:50:26]    |   _____^
[00:50:26]    |  |_____|
[00:50:26]    | ||
[00:50:26] 18 | ||         sum += i;
[00:50:26] 19 | ||     }
[00:50:26]    | ||     ^
[00:50:26]    | ||_____|
[00:50:26]    | |______cannot infer type for `_`
[00:50:26]    |        consider giving `__next` a type
[00:50:26] 
[00:50:26] error: aborting due to previous error
