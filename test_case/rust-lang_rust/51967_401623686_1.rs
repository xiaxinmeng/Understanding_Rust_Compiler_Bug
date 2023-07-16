\n\nMore details can be found in [RFC 438].\n\n[RFC 438]: https://github.com/rust-lang/rfcs/pull/438\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs","byte_start":589,"byte_end":612,"line_start":14,"line_end":14,"column_start":12,"column_end":35,"is_primary":true,"text":[{"text":"    let _: &'static Copy + 'static; //~ ERROR expected a path","highlight_start":12,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs","byte_start":589,"byte_end":612,"line_start":14,"line_end":14,"column_start":12,"column_end":35,"is_primary":true,"text":[{"text":"    let _: &'static Copy + 'static; //~ ERROR expected a path","highlight_start":12,"highlight_end":35}],"label":null,"suggested_replacement":"&'static (Copy + 'static)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0178]: expected a path on the left-hand side of `+`, not `&'static Copy`\n  --> /checkout/src/test/ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs:14:12\n   |\nLL |     let _: &'static Copy + 'static; //~ ERROR expected a path\n   |            ^^^^^^^^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'static (Copy + 'static)`\n\n"}
[00:45:01] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:45:01] {"message":"For more information about this error, try `rustc --explain E0178`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0178`.\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/did_you_mean/trait-object-reference-without-parens-suggestion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/error-codes/E0033-teach.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 4 LL |     let trait_obj: &SomeTrait = Someor more information about this error, try `rustc --explain E0423`.
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033-teach/E0033-teach.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args error-codes/E0033-teach.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0033-teach.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033-teach/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "teach" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033-teach/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] ------------------------------------------
[00:45:01] stderr:
[00:45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"expected value, found trait `SomeTrait`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` varian
[00:45:01] - For more information about an error, try `rustc --explain E0033`.
[00:45:01] 25 
[00:45:01] 
[00:45:01] 
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] The actual stderr differed from the expected stderr.
[00:45:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033/E0033.stderr
[00:45:01] To update references, rerun the tests and pass the `--bless` flag
[00:45:01] To only update this specific test, also pass `--test-args error-codes/E0033.rs`
[00:45:01] error: 1 errors occurred comparing output.
[00:45:01] status: exit code: 101
[00:45:01] status: exit code: 101
[00:45:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0033.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0033/auxiliary" "-A" "unused"
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] ------------------------------------------
[00:45:01] stderr:
[00:45:01] stderr:
[00:45:01] ------------------------------------------
[00:45:01] {"message":"expected value, found trait `SomeTrait`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nForssage":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:01] {"message":"For more information about this error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0423`.\n"}
[00:45:01] ------------------------------------------
[00:45:01] 
[00:45:01] thread '[ui] ui/error-codes/E0033.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:01] 
[00:45:01] 
[00:45:01] ---- [ui] ui/error-festival.rs stdout ----
[00:45:01] diff of stderr:
[00:45:01] 
[00:45:01] 10 LL |     foo::FOO;
[00:45:01] 12 
[00:45:01] 12 
[00:45:01] - error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     x += 2;
[00:45:01] -    |     -^^^^^
[00:45:01] -    |     |
[00:45:01] -    |     cannot use `+=` on type `&str`
[00:45:01] -    |
[00:45:01] -    = note: an implementation of `std::ops::AddAssign` might be missing for `&str`
[00:45:01] + error: aborting due to 2 previous errors
[00:45:01] 22 
[00:45:01] - error[E0599]: no method named `z` found for type `&str` in the current scope
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     x.z();
[00:45:01] - 
[00:45:01] - 
[00:45:01] - error[E0600]: cannot apply unary operator `!` to type `Question`
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     !Question::Yes;
[00:45:01] -    |     ^^^^^^^^^^^^^^ cannot apply unary operator `!`
[00:45:01] -    |
[00:45:01] -    = note: an implementation of `std::ops::Not` might be missing for `Question`
[00:45:01] - 
[00:45:01] - error[E0604]: only `u8` can be cast as `char`, not `u32`
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     0u32 as char;
[00:45:01] - 
[00:45:01] - 
[00:45:01] - error[E0605]: non-primitive cast: `u8` as `std::vec::Vec<u8>`
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     x as Vec<u8>;
[00:45:01] -    |
[00:45:01] -    = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:45:01] - 
[00:45:01] - 
[00:45:01] - error[E0054]: cannot cast as `bool`
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     let x_is_nonzero = x as bool;
[00:45:01] -    |                        ^^^^^^^^^ unsupported cast
[00:45:01] -    |
[00:45:01] -    = help: compare with zero instead
[00:45:01] - 
[00:45:01] - error[E0606]: casting `&u8` as `u32` is invalid
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     let y: u32 = x as u32;
[00:45:01] -    |                  ^^^^^^^^ cannot cast `&u8` as `u32`
[00:45:01] -    |
[00:45:01] - help: did you mean `*x`?
[00:45:01] -    |
[00:45:01] -    |
[00:45:01] - LL |     let y: u32 = x as u32;
[00:45:01] - 
[00:45:01] - 
[00:45:01] - error[E0607]: cannot cstart":5,"highlight_end":6}],"label":"did you mean `x`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `y` in this scope\n  --> /checkout/src/test/ui/error-festival.rs:24:5\n   |\nLL |     y = 2;\n   |     ^ did you mean `x`?\n\n"}
[00:45:01] {"message":"constant `FOO` is private","code":{"code":"E0603","explanation":"\nA private item was used outside its scope.\n\nErroneous code example:\n\n