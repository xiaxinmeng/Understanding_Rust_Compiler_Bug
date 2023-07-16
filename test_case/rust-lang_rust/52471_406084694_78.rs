\n\nIt is also possible to overload most operators for your own type by\nimplementing traits from `std::ops`.\n\nString concatenation appends the string on the right to the string on the\nleft and may require reallocation. This requires ownership of the string\non the left. If something should be added to a string literal, move the\nliteral to the heap by allocating it with `to_owned()` like in\n`\"Your text\".to_owned()`.\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/binary-op-on-double-ref.rs","byte_start":572,"byte_end":577,"line_start":14,"line_end":14,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        x % 2 == 0","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`%` can be used on '{integer}', you can dereference `x`: `*x`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0369]: binary operation `%` cannot be applied to type `&&{integer}`\n  --> /checkout/src/test/ui/binary-op-on-double-ref.rs:14:9\n   |\nLL |         x % 2 == 0\n   |         ^^^^^\n   |\n   = help: `%` can be used on '{integer}', you can dereference `x`: `*x`\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0369`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0369`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/binary-op-on-double-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/block-result/block-must-not-have-result-do.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-do.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n