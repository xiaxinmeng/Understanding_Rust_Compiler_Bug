
[00:53:29] ---- [ui] ui/inference-variable-behind-raw-pointer.rs stdout ----
[00:53:29] 	
[00:53:29] error: ui test compiled successfully!
[00:53:29] status: exit code: 0
[00:53:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference-variable-behind-raw-pointer.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference-variable-behind-raw-pointer.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference-variable-behind-raw-pointer.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:53:29] stdout:
[00:53:29] ------------------------------------------
[00:53:29] 
[00:53:29] ------------------------------------------
[00:53:29] stderr:
[00:53:29] ------------------------------------------
[00:53:29] {"message":"the type of this value must be known in this context","code":{"code":"E0619","explanation":"\nThe type-checker needed to know the type of an expression, but that type had not\nyet been inferred.\n\nErroneous code example:\n\n