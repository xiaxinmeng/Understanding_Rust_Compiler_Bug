\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0442.rs","byte_start":750,"byte_end":800,"line_start":23,"line_end":23,"column_start":5,"column_end":55,"is_primary":true,"text":[{"text":"    fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;","highlight_start":5,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`\n  --> /checkout/src/test/ui/error-codes/E0442.rs:23:5\n   |\nLL |     fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:59:01] {"message":"aborting due to previous error","code"ecific test, also pass `--test-args error-codes/E0443.rs`
[00:59:01] error: 1 errors occurred comparing output.
[00:59:01] status: exit code: 1
[00:59:01] status: exit code: 1
[00:59:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0443.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0443/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0443/auxiliary" "-A" "unused"
[00:59:01] ------------------------------------------
[00:59:01] 
[00:59:01] ------------------------------------------
[00:59:01] stderr:
[00:59:01] stderr:
[00:59:01] ------------------------------------------
[00:59:01] {"message":"unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`","code":{"code":"E0441","explanation":"\nAn unknown platform-specific intrinsic function was used. Erroneous\ncode example:\n\n