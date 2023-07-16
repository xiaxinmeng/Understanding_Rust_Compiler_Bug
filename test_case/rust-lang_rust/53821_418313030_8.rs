\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs","byte_start":829,"byte_end":874,"line_start":33,"line_end":33,"column_start":1,"column_end":46,"is_primary":true,"text":[{"text":"const BAR: Int = unsafe { Foo { r: &42 }.f };","highlight_start":1,"highlight_end":46}],"label":"type validation failed: encountered a pointer, but expected the type u64","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:33:1\n   |\nLL | const BAR: Int = unsafe { Foo { r: &42 }.f };\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type u64\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[00:46:00] {"message":"could not evaluate constant pattern","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs","byte_start":578,"byte_end":581,"line_start":17,"line_end":17,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"        10..=BAR => {}, //~ ERROR lower range bound must be less than or equal to upper","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: could not evaluate constant pattern\n  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:17:14\n   |\nLL |         10..=BAR => {}, //~ ERROR lower range bound must be less than or equal to upper\n   |              ^^^\n\n"}
[00:46:00] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:00] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] thread '[ui] ui/consts/const-eval/ref_to_int_match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:00] 
[00:46:00] 
[00:46:00] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:46:00] diff of stderr:
[00:46:00] 
[00:46:00] 1 error[E0391]: cycle de | ^^^^^^^^^^
[00:46:00] 
[00:46:00] The actual stderr differed from the expected stderr.
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:46:00] To update references, rerun the tests and pass the `--bless` flag
[00:46:00] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:46:00] error: 1 errors occurred comparing output.
[00:46:00] status: exit code: 1
[00:46:00] status: exit code: 1
[00:46:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] ------------------------------------------
[00:46:00] stderr:
[00:46:00] stderr:
[00:46:00] ------------------------------------------
[00:46:00] {"message":"cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n