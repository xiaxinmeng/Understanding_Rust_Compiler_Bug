\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_continue.rs","byte_start":615,"byte_end":623,"line_start":16,"line_end":16,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        continue; //~ ERROR unlabeled `continue` inside of a labeled block","highlight_start":9,"highlight_end":17}],"label":"`continue` statements that would diverge to or through a labeled block need to bear a label","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0695]: unlabeled `continue` inside of a labeled block\n  --> /checkout/src/test/ui/label_break_value_continue.rs:16:9: in fn continue_simple\n   |\nLL |         continue; //~ ERROR unlabeled `continue` inside of a labeled block\n   |         ^^^^^^^^ `continue` statements that would diverge to or through a labeled block need to bear a label\n\n"}
[00:50:37] {"message":"`continue` pointing to a labeled block","code":{"code":"E0696","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_continue.rs","byte_start":809,"byte_end":820,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        continue 'b; //~ ERROR `continue` pointing to a labeled block","highlight_start":9,"highlight_end":20}],"label":"labeled blocks cannot be `continue`'d","suggested_replacement":null,"expansion":null}],"children":[{"message":"labeled block the continue points to","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_continue.rs","byte_start":795,"byte_end":876,"line_start":22,"line_endak_value_continue.rs","byte_start":1019,"byte_end":1027,"line_start":31,"line_end":31,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"            continue; //~ ERROR unlabeled `continue` inside of a labeled block","highlight_start":13,"highlight_end":21}],"label":"`continue` statements that would diverge to or through a labeled block need to bear a label","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0695]: unlabeled `continue` inside of a labeled block\n  --> /checkout/src/test/ui/label_break_value_continue.rs:31:13: in fn continue_crossing\n   |\nLL |             continue; //~ ERROR unlabeled `continue` inside of a labeled block\n   |             ^^^^^^^^ `continue` statements that would diverge to or through a labeled block need to bear a label\n\n"}
[00:50:37] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:50:37] {"message":"Some errors occurred: E0695, E0696.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0695, E0696.\n"}
[00:50:37] {"message":"For more information about an error, try `rustc --explain E0695`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0695`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/label_break_value_continue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/label_break_value_unlabeled_break.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 1 error[E0695]: unlabeled `break` inside of a labeled block
[00:50:37] -   --> $DIR/label_break_value_unlabeled_break.rs:16:9
[00:50:37] +   --> $DIR/label_break_value_unlabeled_break.rs:16:9: in fn unlabeled_break_simple
[00:50:37] 3    |
[00:50:37] 4 LL |         break; //~ ERROR unlabeled `break` inside of a labeled block
[00:50:37] 5    |         ^^^^^ `break` statements that would diverge to or through a labeled block need to bear a label
[00:50:37] 6 
[00:50:37] 6 
[00:50:37] 7 error[E0695]: unlabeled `break` inside of a labeled block
[00:50:37] -   --> $DIR/label_break_value_unlabeled_break.rs:24:13
[00:50:37] +   --> $DIR/label_break_value_unlabeled_break.rs:24:13: in fn unlabeled_break_crossing
[00:50:37] 9    |
[00:50:37] 10 LL |             break; //~ ERROR unlabeled `break` inside of a labeled block
[00:50:37] 11    |             ^^^^^ `break` statements that would diverge to or through a labeled block need to bear a label
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_unlabeled_break.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'label_break_value_unlabeled_break.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bint would diverge to or through a labeled block need to bear a label","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0695]: unlabeled `break` inside of a labeled block\n  --> /checkout/src/test/ui/label_break_value_unlabeled_break.rs:24:13: in fn unlabeled_break_crossing\n   |\nLL |             break; //~ ERROR unlabeled `break` inside of a labeled block\n   |             ^^^^^ `break` statements that would diverge to or through a labeled block need to bear a label\n\n"}
[00:50:37] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0695`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0695`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/label_break_value_unlabeled_break.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/rfc-1937-termination-trait/termination-trait-impl-trait.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 1 error[E0277]: `main` has invalid return type `impl std::marker::Copy`
[00:50:37] -   --> $DIR/termination-trait-impl-trait.rs:12:14
[00:50:37] +   --> $DIR/termination-trait-impl-trait.rs:12:14: in fn main
[00:50:37] 3    |
[00:50:37] 4 LL | fn main() -> impl Copy { }
[00:50:37] 5    |              ^^^^^^^^^ `main` can only return types that implement `std::prer::Copy`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n