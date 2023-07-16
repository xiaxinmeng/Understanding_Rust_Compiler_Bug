\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then -35675.rs","byte_start":1042,"byte_end":1050,"line_start":34,"line_end":34,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"fn bar() -> Variant3 {","highlight_start":13,"highlight_end":21}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can try using the variant's enum","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issue-35675.rs","byte_start":1042,"byte_end":1050,"line_start":34,"line_end":34,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"fn bar() -> Variant3 {","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":"x::Enum","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0412]: cannot find type `Variant3` in this scope\n  --> /checkout/src/test/ui/issue-35675.rs:34:13\n   |\nLL | fn bar() -> Variant3 {\n   |             ^^^^^^^^\n   |             |\n   |             not found in this scope\n   |             help: you can try using the variant's enum: `x::Enum`\n\n"}
[00:51:20] {"message":"expected type, found variant `Some`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-35675.rs","byte_start":1125,"byte_end":1129,"line_start":38,"line_end":38,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"fn qux() -> Some {","highlight_start":13,"highlight_end":17}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expe"text":"struct Foo<T: ?Hash> { }","highlight_start":12,"highlight_end":13}],"label":"unused type parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing `T` or using a marker such as `std::marker::PhantomData`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0392]: parameter `T` is never used\n  --> /checkout/src/test/ui/issue-37534.rs:11:12\n   |\nLL | struct Foo<T: ?Hash> { }\n   |            ^ unused type parameter\n   |\n   = help: consider removing `T` or using a marker such as `std::marker::PhantomData`\n\n"}
[00:51:20] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:20] {"message":"Some errors occurred: E0392, E0405.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0392, E0405.\n"}
[00:51:20] {"message":"For more information about an error, try `rustc --explain E0392`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0392`.\n"}
[00:51:20] ------------------------------------------
[00:51:20] 
[00:51:20] thread '[ui] ui/issue-37534.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:51:20] 
[00:51:20] 
[00:51:20] ---- [ui] ui/issue-38412.rs stdout ----
[00:51:20] diff of stderr:
[00:51:20] 
[00:51:20] 2   --> $DIR/issue-38412.rs:12:9
[00:51:20] 3    |
[00:51:20] 4 LL |     let Box(a) = loop { };
[00:51:20] -    |         ^^^ constructor is not visible here due to private fields
[00:51:20] +    |         ^^^ did you mean `Box { /* fields */ }`?
[00:51:20] 7 error: aborting due to previous error
[00:51:20] 8 
[00:51:20] 
[00:51:20] 
[00:51:20] 
[00:51:20] The actual stderr differed from the expected stderr.
[00:51:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38412/issue-38412.stderr
[00:51:20] To update references, rerun the tests and pass the `--bless` flag
[00:51:20] To only update this specific test, also pass `--test-args issue-38412.rs`
[00:51:20] error: 1 errors occurred comparing output.
[00:51:20] status: exit code: 1
[00:51:20] status: exit code: 1
[00:51:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-38412.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38412/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-38412/auxiliary" "-A" "unused"
[00:51:20] ------------------------------------------
[00:51:20] 
[00:51:20] ------------------------------------------
[00:51:20] stderr:
[00:51:20] stderr:
[00:51:20] ------------------------------------------
[00:51:20] {"message":"expected tuple struct/variant, found struct `Box`","code":{"code":"E0532","explanation":"\nPattern arm did not match expected kind.\n\nErroneous code example:\n\n