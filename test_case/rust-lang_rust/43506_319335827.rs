
...

[00:45:55] ---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----
[00:45:55] 	
[00:45:55] error in revision `rpass2`: compilation failed!
[00:45:55] status: exit code: 101
[00:45:55] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/incremental/remove-private-item-cross-crate/main.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental --target=x86_64-unknown-linux-gnu --cfg rpass2 -Z incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main.inc --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main.stage2-x86_64-unknown-linux-gnu.incremental.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers -Zincremental-info
[00:45:55] stdout:
[00:45:55] ------------------------------------------
[00:45:55] 
[00:45:55] ------------------------------------------
[00:45:55] stderr:
[00:45:55] ------------------------------------------
[00:45:55] thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
[00:45:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:55] {"message":"Error during translation/LLVM phase.","code":null,"level":"error","spans":[],"children":[],"rendered":null}
[00:45:55] 
[00:45:55] ------------------------------------------
[00:45:55] 
[00:45:55] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2499:8
[00:45:55] 
[00:45:55] 
[00:45:55] failures:
[00:45:55]     [incremental] incremental/change_add_field/struct_point.rs
[00:45:55]     [incremental] incremental/change_private_fn/struct_point.rs
[00:45:55]     [incremental] incremental/change_private_impl_method/struct_point.rs
[00:45:55]     [incremental] incremental/change_pub_inherent_method_body/struct_point.rs
[00:45:55]     [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs
[00:45:55]     [incremental] incremental/foreign.rs
[00:45:55]     [incremental] incremental/inlined_hir_34991/main.rs
[00:45:55]     [incremental] incremental/krate-inlined.rs
[00:45:55]     [incremental] incremental/remove-private-item-cross-crate/main.rs
[00:45:55] 
[00:45:55] test result: FAILED. 69 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out
