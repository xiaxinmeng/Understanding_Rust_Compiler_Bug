\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":649,"byte_end":650,"line_start":23,"line_end":23,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x   //~ error: use of moved value: `x`","highlight_start":5,"highlight_end":6}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":735,"byte_end":736,"line_start":26,"line_end":26,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    x;  //~ value moved here","highlight_start":5,"highlight_end":6}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `Int`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/augmented-assignments.rs:23:5\n   |\nLL |     x   //~ error: use of moved value: `x`\n   |     ^ value used here after move\n...\nLL |     x;  //~ value moved here\n   |     - value moved here\n   |\n   = note: move occurs because `x` has type `Int`, which does not implement the `Copy` trait\n\n"}
[00:50:15] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:50:15] {"message":"Some errors occurred: E0382, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0596.\n"}
[00:50:15] {"message":"For more information about an error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0382`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/augmented-assignments.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/blind-item-item-shadow.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/blind-item-item-shadow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/blind-item-item-shadow/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/blind-item-item-shadow/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"the name `foo` is defined multiple times","code":{"code":"E0255","explanation":"\nYou can't import a value whose name is the same as another value defined in the\nmodule.\n\nErroneous code example:\n\n