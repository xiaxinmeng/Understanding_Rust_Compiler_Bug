\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":829,"byte_end":867,"line_start":30,"line_end":30,"column_start":9,"column_end":47,"is_primary":true,"text":[{"text":"        asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));","highlight_start":9,"highlight_end":47}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":789,"byte_end":794,"line_start":27,"line_end":27,"column_start":5,"column_end":10,"is_primary":false,"text":[{"text":"    x = 1;","highlight_start":5,"highlight_end":10}],"label":"first assignment to `x`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x`\n  --> /checkout/src/test/ui/asm-out-assign-imm.rs:30:9: in fn main\n   |\nLL |     x = 1;\n   |     ----- first assignment to `x`\n...\nLL |         asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:48:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:30] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/asm-out-assign-imm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/associated-const-impl-wrong-type.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0326]: implemented const `BAR` has an incompatible type for trait
[00:48:30] -   --> $DIR/associated-const-impl-wrong-type.rs:19:16
[00:48:30] +   --> $DIR/associated-const-impl-wrong-type.rs:19:16: in impl BAR
[00:48:30] 3    |
[00:48:30] 4 LL |     const BAR: u32;
[00:48:30] 5    |                --- type in trait
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const-impl-wrong-type.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'associated-const-impl-wrong-type.rs'
[00:48:30] 
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const-impl-wrong-type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const-impl-wrong-type.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const-impl-wrong-type.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"implemented const `BAR` has an incompatible type for trait","code":{"code":"E0326","explanation":"\nThe types of any associated constants in a trait implementation must match the\ntypes in the trait definition. This error indicates that there was a mismatch.\n\nHere's an example of this error:\n\n