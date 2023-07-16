\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using one of the enum's variants","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::One","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Two","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Three","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Four","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Five","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Six","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Seven","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Eight","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Nine","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs","byte_start":611,"byte_end":623,"line_start":33,"line_end":33,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let z = ManyVariants(); //~ ERROR expected function, found enum","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"ManyVariants::Ten","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0423]: expected function, found enum `ManyVariants`\n  --> /checkout/src/test/ui/did_you_mean/issue-43871-enum-instead-of-variant.rs:33:13\n   |\nLL |     let z = ManyVariants(); //~ ERROR expected function, found enum\n   |             ^^^^^^^^^^^^\nhelp: try using one of the enum's variants\n   |\nLL |     let z = ManyVariants::One(); //~ ERROR expected function, found enum\n   |             ^^^^^^^^^^^^^^^^^\nLL |     let z = ManyVariants::Two(); //~ ERROR expected function, found enum\n   |             ^^^^^^^^^^^^^^^^^\nLL |     let z = ManyVariants::Three(); //~ ERROR expected function, found enum\n   |             ^^^^^^^^^^^^^^^^^^^\nLL |     let z = ManyVariants::Four(); //~ ERROR expected function, found enum\n   |             ^^^^^^^^^^^^^^^^^^\nand 6 other candidates\n\n"}
[01:15:56] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:15:56] {"message":"Some errors occurred: E0423, E0532.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0423, E0532.\n"}
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] thread '[ui] ui/did_you_mean/issue-43871-enum-instead-of-variant.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:15:56] thread '[ui] ui/did_you_mean/issue-43871-enum-instead-of-variant.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:15:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:56] 
[01:15:56] ---- [ui] ui/error-codes/E0063.rs stdout ----
[01:15:56] diff of stderr:
[01:15:56] 
[01:15:56] 10 LL |     let x = PluralFoo {x: 1};
[01:15:56] 11    |             ^^^^^^^^^ missing `y`, `z`
[01:15:56] 12 
[01:15:56] - error[E0063]: missing fields `a`, `b`, `y` and 1 other field in initializer of `TruncatedFoo`
[01:15:56] + error[E0063]: missing fields `y`, `z`, `a` and 1 other field in initializer of `TruncatedFoo`
[01:15:56] 14   --> $DIR/E0063.rs:36:13
[01:15:56] 15    |
[01:15:56] 16 LL |     let y = TruncatedFoo{x:1};
[01:15:56] 
[01:15:56] -    |             ^^^^^^^^^^^^ missing `a`, `b`, `y` and 1 other field
[01:15:56] +    |             ^^^^^^^^^^^^ missing `y`, `z`, `a` and 1 other field
[01:15:56] 18 
[01:15:56] - error[E0063]: missing fields `a`, `b`, `c` and 2 other fields in initializer of `TruncatedPluralFoo`
[01:15:56] + error[E0063]: missing fields `y`, `z`, `a` and 2 other fields in initializer of `TruncatedPluralFoo`
[01:15:56] 20   --> $DIR/E0063.rs:38:13
[01:15:56] 21    |
[01:15:56] 22 LL |     let z = TruncatedPluralFoo{x:1};
[01:15:56] 
[01:15:56] -    |             ^^^^^^^^^^^^^^^^^^ missing `a`, `b`, `c` and 2 other fields
[01:15:56] +    |             ^^^^^^^^^^^^^^^^^^ missing `y`, `z`, `a` and 2 other fields
[01:15:56] 25 error: aborting due to 4 previous errors
[01:15:56] 26 
[01:15:56] 
[01:15:56] 
[01:15:56] 
[01:15:56] The actual stderr differed from the expected stderr.
[01:15:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0063/E0063.stderr
[01:15:56] To update references, rerun the tests and pass the `--bless` flag
[01:15:56] To only update this specific test, also pass `--test-args error-codes/E0063.rs`
[01:15:56] error: 1 errors occurred comparing output.
[01:15:56] status: exit code: 1
[01:15:56] status: exit code: 1
[01:15:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0063.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0063/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0063/auxiliary" "-A" "unused"
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] stderr:
[01:15:56] stderr:
[01:15:56] ------------------------------------------
[01:15:56] {"message":"missing field `x` in initializer of `SingleFoo`","code":{"code":"E0063","explanation":"\nThis error indicates that during an attempt to build a struct or struct-like\nenum variant, one of the fields was not provided. Erroneous code example:\n\n