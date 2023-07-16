
[00:42:07] ---- [ui] ui/const-eval/issue-47971.rs stdout ----
[00:42:07] 	
[00:42:07] error: test compilation failed although it shouldn't!
[00:42:07] status: exit code: 101
[00:42:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/issue-47971.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-47971.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-47971.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:42:07] stdout:
[00:42:07] ------------------------------------------
[00:42:07] 
[00:42:07] ------------------------------------------
[00:42:07] stderr:
[00:42:07] ------------------------------------------
[00:42:07] {"message":"cyclic dependency detected","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n