\n\nAn `if` expression without an `else` block has the type `()`, so this is a type\nerror. To resolve it, add an `else` block having the same type as the `if`\nblock.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/if/if-without-else-result.rs","byte_start":24,"byte_end":40,"line_start":2,"line_end":2,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    let a = if true { true };","highlight_start":13,"highlight_end":29}],"label":"expected (), found bool","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `bool`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0317]: if may be missing an else clause\n  --> /checkout/src/test/ui/if/if-without-else-result.rs:2:13\n   |\nLL |     let a = if true { true };\n   |             ^^^^^^^^^^^^^^^^ expected (), found bool\n   |\n   = note: expected type `()`\n              found type `bool`\n\n"}
[01:14:46] {"message":"For more information about this error, try `rustc --explain E0317`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0317`.\n"}
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] 
---
[01:14:46] 14    |
[01:14:46] 
[01:14:46] 15    = note: expected type `()`
[01:14:46] 16               found type `{integer}`
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] 19 
[01:14:46] 20 error: aborting due to previous error
[01:14:46] 21 
[01:14:46] 
[01:14:46] 
[01:14:46] 
[01:14:46] The actual stderr differed from the expected stderr.
[01:14:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4201/issue-4201.stderr
[01:14:46] To update references, rerun the tests and pass the `--bless` flag
[01:14:46] To only update this specific test, also pass `--test-args issues/issue-4201.rs`
[01:14:46] error: 1 errors occurred comparing output.
[01:14:46] status: exit code: 1
[01:14:46] status: exit code: 1
[01:14:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-4201.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4201/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4201/auxiliary" "-A" "unused"
[01:14:46] ------------------------------------------
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] stderr:
[01:14:46] stderr:
[01:14:46] ------------------------------------------
[01:14:46] {"message":"if may be missing an else clause","code":{"code":"E0317","explanation":"\nThis error occurs when an `if` expression without an `else` block is used in a\ncontext where a type other than `()` is expected, for example a `let`\nexpression:\n\n