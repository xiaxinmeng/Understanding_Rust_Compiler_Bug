plain
[01:34:17] 
[01:34:17] ---- [ui (nll)] ui/static/static-drop-scope.rs stdout ----
[01:34:17] diff of stderr:
[01:34:17] 
[01:34:17] 54 LL |     (x, ()).1
[01:34:17] 55    |     ^^^^^^^ constant functions cannot evaluate destructors
[01:34:17] - error: aborting due to 8 previous errors
[01:34:17] - error: aborting due to 8 previous errors
[01:34:17] + error[E0493]: destructors cannot be evaluated at compile-time
[01:34:17] +    |
[01:34:17] +    |
[01:34:17] + LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
[01:34:17] +    |                                  ^^^^^^^^^^^^^^^^^^^ constants cannot evaluate destructors
[01:34:17] + 
[01:34:17] + error[E0493]: destructors cannot be evaluated at compile-time
[01:34:17] +    |
[01:34:17] +    |
[01:34:17] + LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
[01:34:17] +    |                                           ^^^^^^^^^^^ constants cannot evaluate destructors
[01:34:17] + error: aborting due to 10 previous errors
[01:34:17] 58 
[01:34:17] 59 Some errors occurred: E0493, E0716.
[01:34:17] 60 For more information about an error, try `rustc --explain E0493`.
[01:34:17] 60 For more information about an error, try `rustc --explain E0493`.
[01:34:17] 
[01:34:17] 
[01:34:17] The actual stderr differed from the expected stderr.
[01:34:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope.nll/static-drop-scope.nll.stderr
[01:34:17] To update references, rerun the tests and pass the `--bless` flag
[01:34:17] To only update this specific test, also pass `--test-args static/static-drop-scope.rs`
[01:34:17] error: 1 errors occurred comparing output.
[01:34:17] status: exit code: 1
[01:34:17] status: exit code: 1
[01:34:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope.nll/auxiliary" "-A" "unused"
[01:34:17] ------------------------------------------
[01:34:17] 
[01:34:17] ------------------------------------------
[01:34:17] stderr:
[01:34:17] stderr:
[01:34:17] ------------------------------------------
[01:34:17] {"message":"destructors cannot be evaluated at compile-time","code":{"code":"E0493","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/static/static-drop-scope.rs","byte_start":154,"byte_end":162,"line_start":9,"line_end":9,"column_start":60,"column_end":68,"is_primary":true,"text":[{"text":"static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);","highlight_start":60,"highlight_end":68}],"label":"statics cannot evaluate destructors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0493]: destructors cannot be evaluated at compile-time\n  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60\n   |\nLL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);\n   |                                                            ^^^^^^^^ statics cannot evaluate destructors\n\n"}
[01:34:17] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n