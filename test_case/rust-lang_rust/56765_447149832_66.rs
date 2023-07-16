\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32829-2.rs","byte_start":551,"byte_end":552,"line_start":17,"line_end":17,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        5;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(const_let)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: statements in constants are unstable (see issue #48821)\n  --> /checkout/src/test/ui/issues/issue-32829-2.rs:17:9\n   |\nLL |         5;\n   |         ^\n   |\n   = help: add #![feature(const_let)] to the crate attributes to enable\n\n"}
[00:49:31] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:49:31] {"message":"Some errors occurred: E0015, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0015, E0658.\n"}
[00:49:31] {"message":"For more information about an error, try `rustc --explain E0015`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0015`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-32829-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-40288-2.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 1 error[E0621]: explicit lifetime required in the type of `y`
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL | fn lifetime_transmute_slice<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {
[00:49:31] -    |                                                         -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:49:31] - ...
[00:49:31] - LL |     out[0]
[00:49:31] -    |     ^^^^^^ lifetime `'a` required
[00:49:31] - 
[00:49:31] - error[E0621]: explicit lifetime required in the type of `y`
[00:49:31] 12    |
[00:49:31] 12    |
[00:49:31] 13 LL | fn lifetime_transmute_struct<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {
[00:49:31] 
[00:49:31] 16 LL |     out.head
[00:49:31] 17    |     ^^^^^^^^ lifetime `'a` required
[00:49:31] - error: aborting due to 2 previous errors
[00:49:31] + error: aborting due to previous error
[00:49:31] 20 
[00:49:31] 21 For more information about this error, try `rustc --explain E0621`.
[00:49:31] 21 For more information about this error, try `rustc --explain E0621`.
[00:49:31] 22 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40288-2/issue-40288-2.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-40288-2.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40288-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40288-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40288-2/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n