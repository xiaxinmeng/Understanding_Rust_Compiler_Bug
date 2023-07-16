\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35675.rs","byte_start":575,"byte_end":583,"line_start":24,"line_end":24,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"fn bar() -> Variant3 {","highlight_start":13,"highlight_end":21}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"there is an enum variant `x::Enum::Variant3`; try using the variant's enum","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35675.rs","byte_start":575,"byte_end":583,"line_start":24,"line_end":24,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"fn bar() -> Variant3 {","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":"x::Enum","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0412]: cannot find type `Variant3` in this scope\n  --> /checkout/src/test/ui/issues/issue-35675.rs:24:13\n   |\nLL | fn bar() -> Variant3 {\n   |             ^^^^^^^^ not found in this scope\nhelp: there is an enum variant `x::Enum::Variant3`; try using the variant's enum\n   |\nLL | fn bar() -> x::Enum {\n   |             ^^^^^^^\n\n"}
[01:15:56] {"message":"expected type, found variant `Some`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35675.rs","byte_start":658,"byte_end":662,"line_start":28,"line_end":28,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"fn qux() -> Some {","highlight_start":13,"highlight_end":17}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using the variant's enum","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35675.rs","byte_start":658,"byte_end":662,"line_start":28,"line_end":28,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"fn qux() -> Some {","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"std::option::Option","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0573]: expected type, found variant `Some`\n  --> /checkout/src/test/ui/issues/issue-35675.rs:28:13\n   |\nLL | fn qux() -> Some {\n   |             ^^^^\n   |             |\n   |             not a type\n   |             help: try using the variant's enum: `std::option::Option`\n\n"}
[01:15:56] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[01:15:56] {"message":"Some errors occurred: E0412, E0425, E0573.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0412, E0425, E0573.\n"}
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] thread '[ui] ui/issues/issue-35675.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:15:56] thread '[ui] ui/issues/issue-35675.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:15:56] 
[01:15:56] ---- [ui] ui/issues/issue-7607-1.rs stdout ----
[01:15:56] diff of stderr:
[01:15:56] 
[01:15:56] 2   --> $DIR/issue-7607-1.rs:5:6
[01:15:56] 3    |
[01:15:56] 4 LL | impl Fo {
[01:15:56] -    |      ^^ help: a trait with a similar name exists: `Fn`
[01:15:56] +    |      ^^ help: a struct with a similar name exists: `Foo`
[01:15:56] 7 error: aborting due to previous error
[01:15:56] 8 
[01:15:56] 
[01:15:56] 
[01:15:56] 
[01:15:56] The actual stderr differed from the expected stderr.
[01:15:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/issue-7607-1.stderr
[01:15:56] To update references, rerun the tests and pass the `--bless` flag
[01:15:56] To only update this specific test, also pass `--test-args issues/issue-7607-1.rs`
[01:15:56] error: 1 errors occurred comparing output.
[01:15:56] status: exit code: 1
[01:15:56] status: exit code: 1
[01:15:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7607-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/auxiliary" "-A" "unused"
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] stderr:
[01:15:56] stderr:
[01:15:56] ------------------------------------------
[01:15:56] {"message":"cannot find type `Fo` in this scope","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n