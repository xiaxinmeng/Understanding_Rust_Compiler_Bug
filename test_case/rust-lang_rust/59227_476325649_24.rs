\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0063.rs","byte_start":637,"byte_end":655,"line_start":38,"line_end":38,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let z = TruncatedPluralFoo{x:1};","highlight_start":13,"highlight_end":31}],"label":"missing `y`, `z`, `a` and 2 other fields","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0063]: missing fields `y`, `z`, `a` and 2 other fields in initializer of `TruncatedPluralFoo`\n  --> /checkout/src/test/ui/error-codes/E0063.rs:38:13\n   |\nLL |     let z = TruncatedPluralFoo{x:1};\n   |             ^^^^^^^^^^^^^^^^^^ missing `y`, `z`, `a` and 2 other fields\n\n"}
[01:15:56] {"message":"For more information about this error, try `rustc --explain E0063`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0063`.\n"}
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] 
---
[01:15:56] 
[01:15:56] 63    |             ^^^^
[01:15:56] 64    |             |
[01:15:56] 65    |             not a type
[01:15:56] -    |             help: try using the variant's enum: `Option`
[01:15:56] +    |             help: try using the variant's enum: `std::option::Option`
[01:15:56] 68 error: aborting due to 7 previous errors
[01:15:56] 69 
[01:15:56] 
[01:15:56] 
[01:15:56] 
[01:15:56] The actual stderr differed from the expected stderr.
[01:15:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35675/issue-35675.stderr
[01:15:56] To update references, rerun the tests and pass the `--bless` flag
[01:15:56] To only update this specific test, also pass `--test-args issues/issue-35675.rs`
[01:15:56] error: 1 errors occurred comparing output.
[01:15:56] status: exit code: 1
[01:15:56] status: exit code: 1
[01:15:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35675.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35675/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35675/auxiliary" "-A" "unused"
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] stderr:
[01:15:56] stderr:
[01:15:56] ------------------------------------------
[01:15:56] {"message":"cannot find type `Apple` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n