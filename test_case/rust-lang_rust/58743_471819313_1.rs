\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/block-expression-remove-semicolon.rs","byte_start":55,"byte_end":155,"line_start":6,"line_end":9,"column_start":18,"column_end":6,"is_primary":true,"text":[{"text":"    let x: i32 = {","highlight_start":18,"highlight_end":19},{"text":"        //~^ ERROR mismatched types","highlight_start":1,"highlight_end":36},{"text":"        foo(); //~ HELP consider removing this semicolon","highlight_start":1,"highlight_end":57},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"expected i32, found ()","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `i32`\n   found type `()`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider removing this semicolon","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/block-expression-remove-semicolon.rs","byte_start":106,"byte_end":107,"line_start":8,"line_end":8,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        foo(); //~ HELP consider removing this semicolon","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/block-expression-remove-semicolon.rs:6:18\n   |\nLL |       let x: i32 = {\n   |  __________________^\nLL | |         //~^ ERROR mismatched types\nLL | |         foo(); //~ HELP consider removing this semicolon\n   | |              - help: consider removing this semicolon\nLL | |     };\n   | |_____^ expected i32, found ()\n   |\n   = note: expected type `i32`\n              found type `()`\n\n"}
[01:03:56] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] 
---
[01:03:56] 
[01:03:56] 1 error[E0433]: failed to resolve: partially resolved path in a derive macro
[01:03:56] 2   --> $DIR/issue-46101.rs:3:10
[01:03:56] 3    |
[01:03:56] - LL | #[derive(Foo::Anything)] //~ ERROR failed to resolve: partially resolved path in a derive macro
[01:03:56] + LL | #[derive(Foo::Anything)]
[01:03:56] 5    |          ^^^^^^^^^^^^^ partially resolved path in a derive macro
[01:03:56] 6 
[01:03:56] 7 error[E0601]: `main` function not found in crate `issue_46101`
[01:03:56] 
[01:03:56] The actual stderr differed from the expected stderr.
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46101/issue-46101.stderr
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46101/issue-46101.stderr
[01:03:56] To update references, rerun the tests and pass the `--bless` flag
[01:03:56] To only update this specific test, also pass `--test-args issues/issue-46101.rs`
[01:03:56] error: 1 errors occurred comparing output.
[01:03:56] status: exit code: 1
[01:03:56] status: exit code: 1
[01:03:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46101.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46101/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46101/auxiliary" "-A" "unused"
[01:03:56] ------------------------------------------
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] stderr:
[01:03:56] stderr:
[01:03:56] ------------------------------------------
[01:03:56] {"message":"failed to resolve: partially resolved path in a derive macro","code":{"code":"E0433","explanation":"\nAn undeclared type or module was used.\n\nErroneous code example:\n\n