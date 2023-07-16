\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-49851/compiler-builtins-error.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"//~ ERROR 1:1: 1:1: can't find crate for `core` [E0463]","highlight_start":1,"highlight_end":1}],"label":"can't find crate","suggested_replacement":null,"expansion":null}],"children":[{"message":"the `thumbv7em-none-eabihf` target may not be installed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0463]: can't find crate for `core`\n   |\n   = note: the `thumbv7em-none-eabihf` target may not be installed\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/issue-49851/compiler-builtins-error.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:37] 
[00:50:37] ---- [ui] ui/label_break_value_continue.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 1 error[E0695]: unlabeled `continue` inside of a labeled block
[00:50:37] -   --> $DIR/label_break_value_continue.rs:16:9
[00:50:37] +   --> $DIR/label_break_value_continue.rs:16:9: in fn continue_simple
[00:50:37] 3    |
[00:50:37] 4 LL |         continue; //~ ERROR unlabeled `continue` inside of a labeled block
[00:50:37] 5    |         ^^^^^^^^ `continue` statements that would diverge to or through a labeled block need to bear a label
[00:50:37] 6 
[00:50:37] 6 
[00:50:37] 7 error[E0696]: `continue` pointing to a labeled block
[00:50:37] -   --> $DIR/label_break_value_continue.rs:23:9
[00:50:37] +   --> $DIR/label_break_value_continue.rs:23:9: in fn continue_labeled
[00:50:37] 9    |
[00:50:37] 10 LL |         continue 'b; //~ ERROR `continue` pointing to a labeled block
[00:50:37] 11    |         ^^^^^^^^^^^ labeled blocks cannot be `continue`'d
[00:50:37] 12    |
[00:50:37] 12    |
[00:50:37] 13 note: labeled block the continue points to
[00:50:37] -   --> $DIR/label_break_value_continue.rs:22:5
[00:50:37] +   --> $DIR/label_break_value_continue.rs:22:5: in fn continue_labeled
[00:50:37] 15    |
[00:50:37] 16 LL | /     'b: {
[00:50:37] 17 LL | |         continue 'b; //~ ERROR `continue` pointing to a labeled block
[00:50:37] 19    | |_____^
[00:50:37] 20 
[00:50:37] 20 
[00:50:37] 21 error[E0695]: unlabeled `continue` inside of a labeled block
[00:50:37] -   --> $DIR/label_break_value_continue.rs:31:13
[00:50:37] +   --> $DIR/label_break_value_continue.rs:31:13: in fn continue_crossing
[00:50:37] 23    |
[00:50:37] 24 LL |             continue; //~ ERROR unlabeled `continue` inside of a labeled block
[00:50:37] 25    |             ^^^^^^^^ `continue` statements that would diverge to or through a labeled block need to bear a label
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_continue.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'label_break_value_continue.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label_break_value_continue.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_continue.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_continue.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"unlabeled `continue` inside of a labeled block","code":{"code":"E0695","explanation":"\nA `break` statement without a label appeared inside a labeled block.\n\nExample of erroneous code:\n\n