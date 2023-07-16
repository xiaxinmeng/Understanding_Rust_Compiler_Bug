\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/block-result/block-must-not-have-result-do.rs","byte_start":498,"byte_end":502,"line_start":13,"line_end":13,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        true //~  ERROR mismatched types","highlight_start":9,"highlight_end":13}],"label":"expected (), found bool","suggested_replacement":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `bool`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/block-result/block-must-not-have-result-do.rs:13:9: in fn main\n   |\nLL |         true //~  ERROR mismatched types\n   |         ^^^^ expected (), found bool\n   |\n   = note: expected type `()`\n              found type `bool`\n\n"}
[00:48:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:30] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/block-result/block-must-not-have-result-do.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/block-result/block-must-not-have-result-res.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0308]: mismatched types
[00:48:30] -   --> $DIR/block-must-not-have-result-res.rs:15:9
[00:48:30] +   --> $DIR/block-must-not-have-result-res.rs:15:9: in fn drop::drop
[00:48:30] 3    |
[00:48:30] 4 LL |     fn drop(&mut self) {
[00:48:30] 5    |                        - expected `()` because of default return type
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-res.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'block-result/block-must-not-have-result-res.rs'
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-res.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-res.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-res.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
