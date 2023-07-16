\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":1071,"byte_end":1079,"line_start":38,"line_end":38,"column_start":29,"column_end":37,"is_primary":true,"text":[{"text":"fn paint<C:BoxCar>(c: C, d: C::Color) {","highlight_start":29,"highlight_end":37}],"label":"ambiguous associated type `Color`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":654,"byte_end":665,"line_start":21,"line_end":21,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    type Color;","highlight_start":5,"highlight_end":16}],"label":"ambiguous `Color` from `Box`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs","byte_start":596,"byte_end":607,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    type Color;","highlight_start":5,"highlight_end":16}],"label":"ambiguous `Color` from `Vehicle`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0221]: ambiguous associated type `Color` in bounds of `C`\n  --> /checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs:38:29\n   |\nLL |     type Color;\n   |     ----------- ambiguous `Color` from `Vehicle`\n...\nLL |     type Color;\n   |     ----------- ambiguous `Color` from `Box`\n...\nLL | fn paint<C:BoxCar>(c: C, d: C::Color) {\n   |                             ^^^^^^^^ ambiguous associated type `Color`\n\n"}
[00:50:15] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:50:15] {"message":"Some errors occurred: E0191, E0221.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0221.\n"}
[00:50:15] {"message":"For more information about an error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0191`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/associated-type-projection-from-multiple-supertraits.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/associated-types-ICE-when-projecting-out-of-err.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types-ICE-when-projecting-out-of-err.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-ICE-when-projecting-out-of-err/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-ICE-when-projecting-out-of-err/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"the trait bound `(): Add<A>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n