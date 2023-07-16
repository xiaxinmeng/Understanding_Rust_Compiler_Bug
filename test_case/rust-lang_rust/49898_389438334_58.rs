\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":649,"byte_end":650,"line_start":23,"line_end":23,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x   //~ error: use of moved value: `x`","highlight_start":5,"highlight_end":6}],"label":"value used here after move","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":735,"byte_end":736,"line_start":26,"line_end":26,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    x;  //~ value moved here","highlight_start":5,"highlight_end":6}],"label":"value moved here","suggested_replacement":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `Int`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/augmented-assignments.rs:23:5: in fn main\n   |\nLL |     x   //~ error: use of moved value: `x`\n   |     ^ value used here after move\n...\nLL |     x;  //~ value moved here\n   |     - value moved here\n   |\n   = note: move occurs because `x` has type `Int`, which does not implement the `Copy` trait\n\n"}
[00:48:30] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:30] {"message":"Some errors occurred: E0382, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0596.\n"}
[00:48:30] {"message":"For more information about an error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0382`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/augmented-assignments.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/block-result/block-must-not-have-result-do.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0308]: mismatched types
[00:48:30] -   --> $DIR/block-must-not-have-result-do.rs:13:9
[00:48:30] +   --> $DIR/block-must-not-have-result-do.rs:13:9: in fn main
[00:48:30] 3    |
[00:48:30] 4 LL |         true //~  ERROR mismatched types
[00:48:30] 5    |         ^^^^ expected (), found bool
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'block-result/block-must-not-have-result-do.rs'
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-do.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n