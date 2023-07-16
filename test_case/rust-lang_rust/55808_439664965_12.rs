\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-22560.rs","byte_start":533,"byte_end":612,"line_start":15,"line_end":18,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"type Test = Add +","highlight_start":13,"highlight_end":18},{"text":"            //~^ ERROR E0393","highlight_start":1,"highlight_end":29},{"text":"            //~| ERROR E0191","highlight_start":1,"highlight_end":29},{"text":"            Sub;","highlight_start":1,"highlight_end":16}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/rustc/42741ddb5182fac9cb0afb1f8425ae963b0b9db4/src/libcore/ops/arith.rs","byte_start":2607,"byte_end":2619,"line_start":94,"line_end":94,"column_start":5,"column_end":17,"is_primary":false,"text":[{"text":"","highlight_start":5,"highlight_end":17}],"label":"`Output` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified\n  --> /checkout/src/test/ui/issues/issue-22560.rs:15:13\n   |\nLL |   type Test = Add +\n   |  _____________^\nLL | |             //~^ ERROR E0393\nLL | |             //~| ERROR E0191\nLL | |             Sub;\n   | |_______________^ associated type `Output` must be specified\n\n"}
[00:48:35] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:48:35] {"message":"Some errors occurred: E0191, E0225, E0393.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0225, E0393.\n"}
[00:48:35] {"message":"For more information about an error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0191`.\n"}
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] thread '[ui] ui/issues/issue-22560.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:35] 
[00:48:35] 
[00:48:35] ---- [ui] ui/issues/issue-23024.rs stdout ----
[00:48:35] diff of stderr:
[00:48:35] 
[00:48:35] 17    |
[00:48:35] 18 LL |     println!("{:?}",(vfnfer[0] as Fn)(3));
[00:48:35] 19    |                                   ^^ associated type `Output` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/ops/function.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |     type Output;
[00:48:35] -    |     ------------ `Output` defined here
[00:48:35] 26 error: aborting due to 3 previous errors
[00:48:35] 27 
[00:48:35] 
[00:48:35] 
[00:48:35] 
[00:48:35] The actual stderr differed from the expected stderr.
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/issue-23024.stderr
[00:48:35] To update references, rerun the tests and pass the `--bless` flag
[00:48:35] To only update this specific test, also pass `--test-args issues/issue-23024.rs`
[00:48:35] error: 1 errors occurred comparing output.
[00:48:35] status: exit code: 1
[00:48:35] status: exit code: 1
[00:48:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23024.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23024/auxiliary" "-A" "unused"
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] ------------------------------------------
[00:48:35] stderr:
[00:48:35] stderr:
[00:48:35] ------------------------------------------
[00:48:35] {"message":"the precise format of `Fn`-family traits' type parameters is subject to change. Use parenthetical notation (Fn(Foo, Bar) -> Baz) instead (see issue #29625)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n