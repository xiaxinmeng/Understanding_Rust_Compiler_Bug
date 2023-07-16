\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":889,"byte_end":927,"line_start":33,"line_end":33,"column_start":9,"column_end":47,"is_primary":true,"text":[{"text":"        asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));","highlight_start":9,"highlight_end":47}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":849,"byte_end":854,"line_start":30,"line_end":30,"column_start":5,"column_end":10,"is_primary":false,"text":[{"text":"    x = 1;","highlight_start":5,"highlight_end":10}],"label":"first assignment to `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x`\n  --> /checkout/src/test/ui/asm-out-assign-imm.rs:33:9\n   |\nLL |     x = 1;\n   |     ----- first assignment to `x`\n...\nLL |         asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/asm-out-assign-imm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/associated-const-impl-wrong-type.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const-impl-wrong-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const-impl-wrong-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const-impl-wrong-type/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"implemented const `BAR` has an incompatible type for trait","code":{"code":"E0326","explanation":"\nThe types of any associated constants in a trait implementation must match the\ntypes in the trait definition. This error indicates that there was a mismatch.\n\nHere's an example of this error:\n\n