\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-lifetime.rs","byte_start":544,"byte_end":573,"line_start":18,"line_end":18,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    const NAME: &'a str = \"unit\";","highlight_start":5,"highlight_end":34}],"label":"lifetime mismatch","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&'static str`\n   found type `&'a str`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the lifetime 'a as defined on the impl at 17:6...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-lifetime.rs","byte_start":519,"byte_end":521,"line_start":17,"line_end":17,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"impl<'a> Foo for &'a () {","highlight_start":6,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...does not necessarily outlive the static lifetime","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/associated-const-impl-wrong-lifetime.rs:18:5\n   |\nLL |     const NAME: &'a str = \"unit\";\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch\n   |\n   = note: expected type `&'static str`\n              found type `&'a str`\nnote: the lifetime 'a as defined on the impl at 17:6...\n  --> /checkout/src/test/ui/associated-const-impl-wrong-lifetime.rs:17:6\n   |\nLL | impl<'a> Foo for &'a () {\n   |      ^^\n   = note: ...does not necessarily outlive the static lifetime\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/associated-const-impl-wrong-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/asm-out-assign-imm.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm-out-assign-imm.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm-out-assign-imm/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"cannot assign twice to immutable variable `x`","code":{"code":"E0384","explanation":"\nThis error occurs when an attempt is made to reassign an immutable variable.\nFor example:\n\n