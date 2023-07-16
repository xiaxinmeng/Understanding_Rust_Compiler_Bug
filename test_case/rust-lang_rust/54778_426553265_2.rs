\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17905.rs","byte_start":600,"byte_end":618,"line_start":18,"line_end":18,"column_start":18,"column_end":36,"is_primary":true,"text":[{"text":"    fn say(self: &Pair<&str, isize>) {","highlight_start":18,"highlight_end":36}],"label":"lifetime mismatch","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `Pair<&'_ str, _>`\n   found type `Pair<&str, _>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the lifetime '_ as defined on the impl at 15:5...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17905.rs","byte_start":526,"byte_end":526,"line_start":15,"line_end":15,"column_start":5,"column_end":5,"is_primary":true,"text":[{"text":"    &str, //~ ERROR missing lifetime specifier","highlight_start":5,"highlight_end":5}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 18:5","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17905.rs","byte_start":587,"byte_end":657,"line_start":18,"line_end":20,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn say(self: &Pair<&str, isize>) {","highlight_start":5,"highlight_end":39},{"text":"        println!(\"{}\", self);","highlight_start":1,"highlight_end":30},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched method receiver\n  --> /checkout/src/test/ui/issues/issue-17905.rs:18:18\n   |\nLL |     fn say(self: &Pair<&str, isize>) {\n   |                  ^^^^^^^^^^^^^^^^^^ lifetime mismatch\n   |\n   = note: expected type `Pair<&'_ str, _>`\n              found type `Pair<&str, _>`\nnote: the lifetime '_ as defined on the impl at 15:5...\n  --> /checkout/src/test/ui/issues/issue-17905.rs:15:5\n   |\nLL |     &str, //~ ERROR missing lifetime specifier\n   |     ^\nnote: ...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 18:5\n  --> /checkout/src/test/ui/issues/issue-17905.rs:18:5\n   |\nLL | /     fn say(self: &Pair<&str, isize>) {\nLL | |         println!(\"{}\", self);\nLL | |     }\n   | |_____^\n\n"}
[00:51:12] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: abortingmust\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-19982.rs","byte_start":516,"byte_end":526,"line_start":15,"line_end":15,"column_start":6,"column_end":16,"is_primary":true,"text":[{"text":"impl Fn<(&(),)> for Foo { } //~ ERROR missing lifetime specifier","highlight_start":6,"highlight_end":16}],"label":"expected an `FnMut<(&(),)>` closure, found `Foo`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::ops::FnMut<(&(),)>` is not implemented for `Foo`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: expected a `std::ops::FnMut<(&(),)>` closure, found `Foo`\n  --> /checkout/src/test/ui/issues/issue-19982.rs:15:6\n   |\nLL | impl Fn<(&(),)> for Foo { } //~ ERROR missing lifetime specifier\n   |      ^^^^^^^^^^ expected an `FnMut<(&(),)>` closure, found `Foo`\n   |\n   = help: the trait `std::ops::FnMut<(&(),)>` is not implemented for `Foo`\n\n"}
[00:51:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:12] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] thread '[ui] ui/issues/issue-19982.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:12] 
[00:51:12] 
[00:51:12] ---- [ui] ui/underscore-lifetime/underscore-lifetime-binders.rs stdout ----
[00:51:12] diff of stderr:
[00:51:12] 
[00:51:12] 4 LL | struct Baz<'a>(&'_ &'a u8); //~ ERROR missing lifetime specifier
[00:51:12] 5    |                 ^^ expected lifetime parameter
[00:51:12] - error[E0106]: missing lifetime specifier
[00:51:12] -   --> $DIR/underscore-lifetime-binders.rs:14:10
[00:51:12] -    |
[00:51:12] -    |
[00:51:12] - LL | impl Foo<'_> { //~ ERROR missing lifetime specifier
[00:51:12] -    |          ^^ expected lifetime parameter
[00:51:12] - 
[00:51:12] 13 error[E0262]: invalid lifetime parameter name: `'_`
[00:51:12] 15    |
[00:51:12] 
[00:51:12] 39    |
[00:51:12] 39    |
[00:51:12] 40    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `y`
[00:51:12] - error: aborting due to 6 previous errors
[00:51:12] + error: aborting due to 5 previous errors
[00:51:12] 43 
[00:51:12] 44 Some errors occurred: E0106, E0262.
[00:51:12] 44 Some errors occurred: E0106, E0262.
[00:51:12] 45 For more information about an error, try `rustc --explain E0106`.
[00:51:12] 
[00:51:12] 
[00:51:12] The actual stderr differed from the expected stderr.
[00:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/underscore-lifetime-binders.stderr
[00:51:12] To update references, rerun the tests and pass the `--bless` flag
[00:51:12] To only update this specific test, also pass `--test-args underscore-lifetime/underscore-lifetime-binders.rs`
[00:51:12] error: 1 errors occurred comparing output.
[00:51:12] status: exit code: 1
[00:51:12] status: exit code: 1
[00:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/auxiliary" "-A" "unused"
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] ------------------------------------------
[00:51:12] stderr:
[00:51:12] stderr:
[00:51:12] ------------------------------------------
[00:51:12] {"message":"missing lifetime specifier","code":{"code":"E0106","explanation":"\nThis error indicates that a lifetime is missing from a type. If it is an error\ninside a function signature, the problem may be with failing to adhere to the\nlifetime elision rules (see below).\n\nHere are some simple examples of where you'll run into this error:\n\n