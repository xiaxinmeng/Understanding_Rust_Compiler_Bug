\n\nAn `if` expression without an `else` block has the type `()`, so this is a type\nerror. To resolve it, add an `else` block having the same type as the `if`\nblock.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/if/if-without-else-as-fn-expr.rs","byte_start":316,"byte_end":349,"line_start":17,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if bar % 5 == 0 {","highlight_start":5,"highlight_end":22},{"text":"        3","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":"expected (), found usize","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `usize`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0317]: if may be missing an else clause\n  --> /checkout/src/test/ui/if/if-without-else-as-fn-expr.rs:17:5\n   |\nLL | /     if bar % 5 == 0 {\nLL | |         3\nLL | |     }\n   | |_____^ expected (), found usize\n   |\n   = note: expected type `()`\n              found type `usize`\n\n"}
[01:14:46] {"message":"For more information about this error, try `rustc --explain E0317`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0317`.\n"}
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] 
---
[01:14:46] 9    |
[01:14:46] 10    = note: expected type `()`
[01:14:46] 11               found type `bool`
[01:14:46] 
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] 14 
[01:14:46] 15 error: aborting due to previous error
[01:14:46] 16 
[01:14:46] 
[01:14:46] 
[01:14:46] 
[01:14:46] The actual stderr differed from the expected stderr.
[01:14:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-result/if-without-else-result.stderr
[01:14:46] To update references, rerun the tests and pass the `--bless` flag
[01:14:46] To only update this specific test, also pass `--test-args if/if-without-else-result.rs`
[01:14:46] error: 1 errors occurred comparing output.
[01:14:46] status: exit code: 1
[01:14:46] status: exit code: 1
[01:14:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-without-else-result.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-result/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-result/auxiliary" "-A" "unused"
[01:14:46] ------------------------------------------
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] stderr:
[01:14:46] stderr:
[01:14:46] ------------------------------------------
[01:14:46] {"message":"if may be missing an else clause","code":{"code":"E0317","explanation":"\nThis error occurs when an `if` expression without an `else` block is used in a\ncontext where a type other than `()` is expected, for example a `let`\nexpression:\n\n