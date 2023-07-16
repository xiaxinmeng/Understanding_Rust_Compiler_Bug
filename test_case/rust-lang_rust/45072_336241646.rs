
[00:52:16] stderr:
[00:52:16] ------------------------------------------
[00:52:16] error[E0282]: type annotations needed
[00:52:16]   --> /checkout/src/test/run-pass/closure-expected-type/expect-infer-supply-two-infers.rs:17:30
[00:52:16]    |
[00:52:16] 17 |     with_closure(|mut x: Vec<_>, y| {
[00:52:16]    |                              ^ cannot infer type for `_`
[00:52:16] 
[00:52:16] error: aborting due to previous error
[00:52:16] 
[00:52:16] 
[00:52:16] ------------------------------------------
[00:52:16] 
[00:52:16] thread '[run-pass] run-pass/closure-expected-type/expect-infer-supply-two-infers.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:52:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:16] 
[00:52:16] ---- [run-pass] run-pass/closure-expected-type/supply-just-return-type.rs stdout ----
[00:52:16] 	
[00:52:16] error: compilation failed!
[00:52:16] status: exit code: 101
[00:52:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/closure-expected-type/supply-just-return-type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/closure-expected-type/supply-just-return-type.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/closure-expected-type/supply-just-return-type.stage2-x86_64-unknown-linux-gnu.run-pass.libaux"
[00:52:16] stdout:
[00:52:16] ------------------------------------------
[00:52:16] 
[00:52:16] ------------------------------------------
[00:52:16] stderr:
[00:52:16] ------------------------------------------
[00:52:16] error[E0282]: type annotations needed
[00:52:16]   --> /checkout/src/test/run-pass/closure-expected-type/supply-just-return-type.rs:33:31
[00:52:16]    |
[00:52:16] 33 |     let z = with_closure(|x: &_| -> Result<char, ()> { Ok(*x) });
[00:52:16]    |                               ^ cannot infer type for `_`
[00:52:16] 
[00:52:16] error: aborting due to previous error
[00:52:16] 
[00:52:16] 
