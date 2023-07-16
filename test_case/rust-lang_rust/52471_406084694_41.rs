\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-type.rs","byte_start":562,"byte_end":565,"line_start":19,"line_end":19,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    const BAR: i32 = -1;","highlight_start":16,"highlight_end":19}],"label":"expected u32, found i32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-type.rs","byte_start":495,"byte_end":498,"line_start":13,"line_end":13,"column_start":16,"column_end":19,"is_primary":false,"text":[{"text":"    const BAR: u32;","highlight_start":16,"highlight_end":19}],"label":"type in trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0326]: implemented const `BAR` has an incompatible type for trait\n  --> /checkout/src/test/ui/associated-const-impl-wrong-type.rs:19:16\n   |\nLL |     const BAR: u32;\n   |                --- type in trait\n...\nLL |     const BAR: i32 = -1;\n   |                ^^^ expected u32, found i32\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0326`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0326`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/associated-const-impl-wrong-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/associated-type-projection-from-multiple-supertraits.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-projection-from-multiple-supertraits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-projection-from-multiple-supertraits/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"ambiguous associated type `Color` in bounds of `C`","code":{"code":"E0221","explanation":"\nAn attempt was made to retrieve an associated type, but the type was ambiguous.\nFor example:\n\n