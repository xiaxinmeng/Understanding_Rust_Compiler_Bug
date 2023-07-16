\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/traits/trait-alias-object-type.rs","byte_start":744,"byte_end":759,"line_start":24,"line_end":24,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let c: &dyn I32Iterator = &vec![123].into_iter();","highlight_start":13,"highlight_end":28}],"label":"missing associated type `Item` value","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified\n  --> /checkout/src/test/run-pass/traits/trait-alias-object-type.rs:24:13\n   |\nLL |     let c: &dyn I32Iterator = &vec![123].into_iter();\n   |             ^^^^^^^^^^^^^^^ missing associated type `Item` value\n\n"}
[00:57:41] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:41] {"message":"For more information about this error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0191`.\n"}
[00:57:41] ------------------------------------------
[00:57:41] 
[00:57:41] thread '[run-pass] run-pass/traits/trait-alias-object-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:57:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:41] 
[00:57:41] ---- [run-pass] run-pass/traits/trait-alias-bounds.rs stdout ----
[00:57:41] 
[00:57:41] error: test compilation failed although it shouldn't!
[00:57:41] status: exit code: 1
[00:57:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/traits/trait-alias-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-bounds/auxiliary"
[00:57:41] ------------------------------------------
[00:57:41] 
[00:57:41] ------------------------------------------
[00:57:41] stderr:
[00:57:41] stderr:
[00:57:41] ------------------------------------------
[00:57:41] {"message":"binary operation `==` cannot be applied to type `impl SendEqAlias<i32>`","code":{"code":"E0369","explanation":"\nA binary operation was attempted on a type which doesn't support it.\nErroneous code example:\n\n